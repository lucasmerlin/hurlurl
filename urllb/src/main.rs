#[macro_use]
extern crate diesel;

use crate::db::Pool;
use crate::error::Error;
use crate::models::{CreateLinkDto, LinkDto};
use crate::service::{
    create_link, get_link_and_targets, increase_redirect_count, set_link_payment_status,
};
use axum::body::{Empty, Full};
use axum::extract::{Path, State};
use axum::http::{header, HeaderValue};
use axum::response::{Redirect, Response};
use axum::routing::get_service;
use axum::{
    body,
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Extension, Json, Router,
};
use axum_client_ip::{SecureClientIp, SecureClientIpSource};
use include_dir::{include_dir, Dir};
use lazy_static::lazy_static;
use nanoid::nanoid;
use rand::seq::SliceRandom;
use serde::{Deserialize, Serialize};
use shared::{CreateResult, PaymentStatus};
use std::fmt::Debug;
use std::net::SocketAddr;
use std::str::FromStr;
use std::sync::Arc;
use stripe::{
    CheckoutSession, CheckoutSessionId, CheckoutSessionMode, CreateCheckoutSession,
    CreateCheckoutSessionLineItems,
};
use tokio::io;
use tower_http::services::ServeDir;
use tracing_subscriber::EnvFilter;
use validator::Validate;

mod db;
mod error;
mod models;
mod schema;
mod service;
mod stats;

static STATIC_DIR: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/../web/dist");

#[derive(serde::Deserialize)]
struct Config {
    ip_source: SecureClientIpSource,
    database_url: String,
    stripe_secret_key: String,
    stripe_price_id: String,
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")),
        )
        .init();

    let config: Arc<Config> = Arc::new(envy::from_env().unwrap());

    let pool = db::connect_and_migrate(&config.database_url)
        .await
        .expect("Failed to connect to database");

    let stripe_client = stripe::Client::new(config.stripe_secret_key.clone());

    let serve_dir_service = get_service(
        ServeDir::new(option_env!("STATIC_DIR").unwrap_or("../web/dist"))
            .precompressed_gzip()
            .append_index_html_on_directories(true),
    )
    .handle_error(|error: io::Error| async move {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Unhandled internal error: {error}"),
        )
    });

    let static_router = Router::new().route("/*path", serve_dir_service.clone());

    let app = Router::new()
        .route("/", serve_dir_service.clone())
        .route("/api/stats", get(total_stats))
        .route("/info/*path", get(root))
        .route("/api/links", post(post_link))
        .route("/api/links/:link", get(link_info))
        .nest("/static", static_router)
        .route("/:link", get(link).post(post_link))
        .with_state(pool)
        .layer(Extension(stripe_client))
        .layer(config.ip_source.clone().into_extension())
        .layer(Extension(config.clone()));

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    tracing::info!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service_with_connect_info::<SocketAddr>())
        .await
        .unwrap();
}

async fn root() -> impl IntoResponse {
    static_path(Path("index.html".to_string())).await
}

async fn link(
    Path(params): Path<Params>,
    State(pool): State<Pool>,
) -> Result<Response, StatusCode> {
    let mut connection = pool
        .get()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let (link, target_results) = get_link_and_targets(&mut connection, &params.link)
        .await
        .map_err(|_| StatusCode::NOT_FOUND)?;

    if let Some(payment_status) = &link.payment_status {
        match payment_status {
            PaymentStatus::Pending | PaymentStatus::Failed => {
                return Err(StatusCode::NOT_FOUND);
            }
            PaymentStatus::Succeeded => {}
        }
    }

    if link.fraud {
        return Ok(Response::builder()
            .status(StatusCode::UNAVAILABLE_FOR_LEGAL_REASONS)
            .body(body::boxed(format!(
                "The link you are trying to access has been marked as fraudulent. 
It was probably used in a phishing attack. 
                
                
The following reason was given: {}",
                link.fraud_reason.as_deref().unwrap_or("No reason")
            )))
            .unwrap());
    }

    // select random target from results
    let target = target_results.choose(&mut rand::thread_rng());

    if let Some(target) = target {
        increase_redirect_count(&mut connection, &link, target)
            .await
            .ok();

        if link.permanent_redirect {
            Ok(Redirect::permanent(&target.target_url).into_response())
        } else {
            Ok(Redirect::temporary(&target.target_url).into_response())
        }
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}

#[derive(Debug, Deserialize, Serialize)]
struct Params {
    link: String,
}

lazy_static! {
    static ref BLACKLIST: Vec<String> = include_str!("blacklist.txt")
        .lines()
        .map(|s| s.to_string())
        .collect();
}

lazy_static! {
    static ref WHITELIST: Vec<String> = include_str!("whitelist.txt")
        .lines()
        .map(|s| s.to_string())
        .collect();
}

async fn post_link(
    State(pool): State<Pool>,
    Extension(stripe): Extension<stripe::Client>,
    Extension(config): Extension<Arc<Config>>,
    SecureClientIp(ip): SecureClientIp,
    Json(body): Json<CreateLinkDto>,
) -> Result<impl IntoResponse, StatusCode> {
    body.validate().map_err(|_| StatusCode::BAD_REQUEST)?;

    if BLACKLIST.iter().any(|b| {
        body.targets
            .iter()
            .map(|t| &t.target_url)
            .any(|t| t.contains(b))
    }) {
        return Err(StatusCode::FORBIDDEN);
    }

    let whitelisted = WHITELIST.iter().any(|w| {
        body.targets
            .iter()
            .map(|t| &t.target_url)
            .any(|t| t.starts_with(w))
    });

    let url = nanoid!(5);

    let success_url = format!("https://hurlurl.com/info/{url}");

    let session = if !whitelisted {
        let create_session = CreateCheckoutSession {
            line_items: Some(vec![CreateCheckoutSessionLineItems {
                price: Some(config.stripe_price_id.clone()),
                quantity: Some(1),
                ..Default::default()
            }]),
            mode: Some(CheckoutSessionMode::Payment),
            success_url: Some(&success_url),
            cancel_url: Some("https://hurlurl.com"),
            ..Default::default()
        };

        let session = CheckoutSession::create(&stripe, create_session)
            .await
            .map_err(error::Error::StripeError)?;
        Some(session)
    } else {
        None
    };

    let mut connection = pool.get().await.map_err(Error::PoolError)?;
    let (link, target_results) = create_link(
        &mut connection,
        &body,
        &url,
        ip.into(),
        session.as_ref().map(|s| s.id.to_string()),
    )
    .await?;

    match session {
        None => Ok(Json(CreateResult::Link(LinkDto {
            link,
            targets: target_results,
        }))),
        Some(session) => Ok(Json(CreateResult::StripeRedirect(
            session.url.ok_or(StatusCode::INTERNAL_SERVER_ERROR)?,
        ))),
    }
}

async fn link_info(
    Path(params): Path<Params>,
    State(pool): State<Pool>,
    Extension(stripe): Extension<stripe::Client>,
) -> Result<impl IntoResponse, StatusCode> {
    let mut connection = pool
        .get()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let (mut link, results) = get_link_and_targets(&mut connection, &params.link)
        .await
        .map_err(|_| StatusCode::NOT_FOUND)?;

    if let Some(status) = &link.payment_status {
        match status {
            PaymentStatus::Pending => {
                if let Some(id) = &link.stripe_session_id {
                    let session = CheckoutSession::retrieve(
                        &stripe,
                        &CheckoutSessionId::from_str(id).unwrap(),
                        &[],
                    )
                    .await
                    .map_err(Error::StripeError)?;

                    if session.status == Some(stripe::CheckoutSessionStatus::Complete) {
                        set_link_payment_status(
                            &mut connection,
                            &params.link,
                            PaymentStatus::Succeeded,
                        )
                        .await?;
                    } else {
                        set_link_payment_status(
                            &mut connection,
                            &params.link,
                            PaymentStatus::Failed,
                        )
                        .await?;
                        return Err(StatusCode::NOT_FOUND);
                    }
                }
            }
            PaymentStatus::Failed => {
                return Err(StatusCode::NOT_FOUND);
            }
            PaymentStatus::Succeeded => {}
        }
    }

    link.stripe_session_id = None;

    Ok(Json(LinkDto {
        link,
        targets: results,
    }))
}

async fn total_stats(State(pool): State<Pool>) -> Result<impl IntoResponse, StatusCode> {
    let mut connection = pool
        .get()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let stats = stats::total_stats(&mut connection)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(stats))
}

async fn static_path(Path(path): Path<String>) -> impl IntoResponse {
    let path = path.trim_start_matches('/');
    let mime_type = mime_guess::from_path(path).first_or_text_plain();

    match STATIC_DIR.get_file(path) {
        None => Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(body::boxed(Empty::new()))
            .unwrap(),
        Some(file) => {
            let mut response = Response::builder().status(StatusCode::OK).header(
                header::CONTENT_TYPE,
                HeaderValue::from_str(mime_type.as_ref()).unwrap(),
            );

            if mime_type != "text/html" {
                response = response.header(
                    header::CACHE_CONTROL,
                    HeaderValue::from_static("public, max-age=31536000"),
                );
            };

            response
                .body(body::boxed(Full::from(file.contents())))
                .unwrap()
        }
    }
}

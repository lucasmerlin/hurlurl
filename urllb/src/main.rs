#[macro_use]
extern crate diesel;

use std::env;
use std::net::SocketAddr;

use axum::body::{Empty, Full};
use axum::extract::Path;
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

use diesel_async::pooled_connection::AsyncDieselConnectionManager;

use include_dir::{include_dir, Dir};
use rand::seq::SliceRandom;
use serde::{Deserialize, Serialize};
use tokio::io;
use tower_http::services::ServeDir;
use validator::Validate;

use crate::db::{old_connection, run_migrations};
use crate::models::{CreateLinkDto, LinkDto};

use crate::db::Pool;
use crate::service::{create_link, get_link_and_targets, increase_redirect_count};
mod db;
mod models;
mod schema;
mod service;
mod stats;

static STATIC_DIR: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/../web/dist");

#[tokio::main]
async fn main() {
    {
        let mut db = old_connection();
        run_migrations(&mut db);
    }

    tracing_subscriber::fmt::init();

    let manager = AsyncDieselConnectionManager::new(
        env::var("DATABASE_URL").expect("DATABASE_URL must be set"),
    );

    let pool = Pool::builder().build(manager).await.unwrap();

    let serve_dir_service = get_service(
        ServeDir::new(option_env!("STATIC_DIR").unwrap_or("../web/dist"))
            .precompressed_gzip()
            .append_index_html_on_directories(true),
    )
    .handle_error(|error: io::Error| async move {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Unhandled internal error: {}", error),
        )
    });

    let app = Router::new()
        .route("/", serve_dir_service.clone())
        .route("/api/stats", get(total_stats))
        .route("/info/*path", get(root))
        .route("/api/links", post(post_link))
        .route("/api/links/:link", get(link_info))
        .nest("/static", serve_dir_service)
        .route("/:link", get(link).post(post_link))
        .layer(Extension(pool));

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn root() -> impl IntoResponse {
    static_path(Path("index.html".to_string())).await
}

async fn link(
    Path(params): Path<Params>,
    Extension(pool): Extension<Pool>,
) -> Result<impl IntoResponse, StatusCode> {
    let mut connection = pool
        .get()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let (link, target_results) = get_link_and_targets(&mut connection, &params.link)
        .await
        .map_err(|_| StatusCode::NOT_FOUND)?;

    // select random target from results
    let target = target_results.choose(&mut rand::thread_rng());

    if let Some(target) = target {
        increase_redirect_count(&mut connection, &link, target)
            .await
            .ok();

        if link.permanent_redirect {
            Ok(Redirect::permanent(&target.target_url))
        } else {
            Ok(Redirect::temporary(&target.target_url))
        }
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}

#[derive(Debug, Deserialize, Serialize)]
struct Params {
    link: String,
}

async fn post_link(
    Json(body): Json<CreateLinkDto>,
    Extension(pool): Extension<Pool>,
) -> Result<impl IntoResponse, StatusCode> {
    body.validate().map_err(|_| StatusCode::BAD_REQUEST)?;

    let mut connection = pool
        .get()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let (link, target_results) = create_link(&mut connection, &body)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(LinkDto {
        link,
        targets: target_results,
    }))
}

async fn link_info(
    Path(params): Path<Params>,
    Extension(pool): Extension<Pool>,
) -> Result<impl IntoResponse, StatusCode> {
    let mut connection = pool
        .get()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let (link, results) = get_link_and_targets(&mut connection, &params.link)
        .await
        .map_err(|_| StatusCode::NOT_FOUND)?;

    Ok(Json(LinkDto {
        link,
        targets: results,
    }))
}

async fn total_stats(Extension(pool): Extension<Pool>) -> Result<impl IntoResponse, StatusCode> {
    let mut connection = pool.get().await.map_err(|_| StatusCode::NOT_FOUND)?;

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

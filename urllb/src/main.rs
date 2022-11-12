#[macro_use]
extern crate diesel;

use std::net::SocketAddr;

use axum::{body, http::StatusCode, Json, response::IntoResponse, Router, routing::{get, post}};
use axum::body::{Empty, Full};
use axum::extract::Path;
use axum::http::{header, HeaderValue};
use axum::response::{Redirect, Response};
use diesel::{Connection, QueryDsl};
use diesel::associations::HasTable;
use diesel::expression_methods::ExpressionMethods;
use diesel_async::RunQueryDsl;
use diesel_migrations::MigrationHarness;
use include_dir::{Dir, include_dir};
use nanoid::nanoid;
use rand::seq::SliceRandom;
use serde::{Deserialize, Serialize};
use tower_http::cors;
use tower_http::cors::CorsLayer;
use validator::Validate;

use crate::db::{db, old_connection, run_migrations};
use crate::models::{CreateLinkDto, Link, LinkDto, NewLink, NewTarget, Target};
use crate::schema::links::dsl::*;
use crate::schema::links::url;
use crate::schema::targets::dsl::targets;
use crate::schema::targets::link_id;

mod db;
mod models;
mod schema;

static STATIC_DIR: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/../web/dist");

#[tokio::main]
async fn main() {

    // run migrations
    {
        let mut db = old_connection();
        run_migrations(&mut db);
    }

    // initialize tracing
    tracing_subscriber::fmt::init();


    let cors = CorsLayer::new()
        // allow requests from any origin
        .allow_methods(cors::Any)
        .allow_headers(cors::Any)
        .allow_origin(cors::Any);

    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/", get(root))
        .route("/info/*path", get(root))
        .route("/api/links", post(post_link))
        .route("/api/links/:link", get(link_info))
        .route("/static/*path", get(static_path))
        .route("/:link", get(link).post(post_link))
        .layer(cors);

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

// basic handler that responds with a static string
async fn root() -> impl IntoResponse {
    static_path(Path("index.html".to_string())).await
}

async fn link(Path(params): Path<Params>) -> Result<impl IntoResponse, StatusCode> {
    let mut db = db().await;
    let link: Link = links.filter(url.eq(params.link))
        .first::<Link>(&mut db)
        .await
        .map_err(|_| StatusCode::NOT_FOUND)?;

    let results: Vec<Target> = targets.filter(link_id.eq(link.id))
        .limit(10)
        .load::<Target>(&mut db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // select random target from results
    let target = results.choose(&mut rand::thread_rng());

    if let Some(target) = target {
        diesel::update(&link)
            .set(schema::links::redirects.eq(schema::links::redirects + 1))
            .execute(&mut db)
            .await
            .ok();

        diesel::update(target)
            .set(schema::targets::redirects.eq(schema::targets::redirects + 1))
            .execute(&mut db)
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

async fn post_link(Json(body): Json<CreateLinkDto>) -> Result<impl IntoResponse, StatusCode> {
    body.validate().map_err(|_| StatusCode::BAD_REQUEST)?;

    let mut db = db().await;

    let link = NewLink {
        url: &body.url.unwrap_or_else(|| nanoid!(5)),
        permanent_redirect: body.permanent_redirect,
    };

    let link = diesel::insert_into(links::table())
        .values(&link)
        .get_result::<Link>(&mut db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let target_results = diesel::insert_into(targets::table())
        .values(
            &body.targets.iter().map(|target| {
                NewTarget {
                    link_id: link.id,
                    target_url: &target.target_url,
                }
            }).collect::<Vec<_>>()
        )
        .get_results::<Target>(&mut db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(LinkDto {
        link,
        targets: target_results,
    }))
}

async fn link_info(Path(params): Path<Params>) -> Result<impl IntoResponse, StatusCode> {
    let mut db = db().await;
    let link: Link = links.filter(url.eq(params.link))
        .first::<Link>(&mut db)
        .await
        .map_err(|_| StatusCode::NOT_FOUND)?;

    let results: Vec<Target> = targets.filter(link_id.eq(link.id))
        .limit(10)
        .load::<Target>(&mut db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(LinkDto {
        link,
        targets: results,
    }))
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
            let mut response = Response::builder()
                .status(StatusCode::OK)
                .header(
                    header::CONTENT_TYPE,
                    HeaderValue::from_str(mime_type.as_ref()).unwrap(),
                );

            if mime_type != "text/html" {
                response = response.header(
                    header::CACHE_CONTROL,
                    HeaderValue::from_static("public, max-age=31536000"),
                );
            };

            response.body(body::boxed(Full::from(file.contents())))
                .unwrap()
        }
    }
}

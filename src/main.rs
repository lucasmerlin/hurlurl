#[macro_use]
extern crate diesel;

use std::net::SocketAddr;
use std::ptr::eq;
use std::sync::Arc;

use axum::{Extension, http::StatusCode, Json, response::IntoResponse, Router, routing::{get, post}};
use axum::extract;
use axum::extract::Path;
use axum::response::Redirect;
use diesel::associations::HasTable;
use diesel::expression_methods::ExpressionMethods;
use diesel::{Connection, PgConnection, QueryDsl};
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use nanoid::nanoid;
use serde::{Deserialize, Serialize};

use crate::db::{db, old_connection, run_migrations};
use crate::models::{CreateLinkDto, Link, LinkDto, NewLink, NewTarget, Target};
use crate::schema::links::dsl::*;
use crate::schema::links::url;
use crate::schema::targets::dsl::targets;
use crate::schema::targets::link_id;
use diesel_migrations::{embed_migrations, EmbeddedMigrations};

mod db;
mod models;
mod schema;

#[tokio::main]
async fn main() {

    let mut db = old_connection();
    run_migrations(&mut db);

    // initialize tracing
    tracing_subscriber::fmt::init();

    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/", get(root).post(post_link))
        .route("/:link", get(link).post(post_link));

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
async fn root() -> &'static str {
    "Hello, World! Yay"
}

async fn link(Path(params): Path<Params>) -> Result<impl IntoResponse, StatusCode> {
    let mut db = db().await;
    let link = links.filter(url.eq(params.link))
        .first::<Link>(&mut db)
        .await
        .map_err(|_| StatusCode::NOT_FOUND)?;

    let results = targets.filter(link_id.eq(link.id))
        .limit(10)
        .load::<Target>(&mut db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // select random target from results
    let target = &results[rand::random::<usize>() % results.len()];

    if link.permanent_redirect {
        Ok(Redirect::permanent(&target.target_url))
    } else {
        Ok(Redirect::temporary(&target.target_url))
    }
}

#[derive(Debug, Deserialize, Serialize)]
struct Params {
    link: String,
}

async fn post_link(Json(body): Json<CreateLinkDto>) -> Result<impl IntoResponse, StatusCode> {
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
                    target_url: target,
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

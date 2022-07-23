extern crate dotenv;

use diesel::prelude::*;
use dotenv::dotenv;
use std::env;
use diesel::{QueryDsl, RunQueryDsl, Table};
use diesel_async::{AsyncConnection, AsyncPgConnection};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};


pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

pub async fn db() -> AsyncPgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    AsyncPgConnection::establish(&database_url).await.expect("Error connecting to database")
}

pub fn old_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).unwrap()
}

pub fn run_migrations(db: &mut PgConnection) {
    db.run_pending_migrations(MIGRATIONS).unwrap();
}

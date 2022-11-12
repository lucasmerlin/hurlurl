extern crate dotenv;

use diesel::prelude::*;

use diesel_async::AsyncPgConnection;
use diesel_migrations::{
    embed_migrations, EmbeddedMigrations, HarnessWithOutput, MigrationHarness,
};
use dotenv::dotenv;

use bb8::{Pool as BB8Pool, PooledConnection};
use diesel::Connection as DieselConnection;
use diesel_async::pooled_connection::AsyncDieselConnectionManager;
use std::env;
use std::io::stdout;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

pub type Pool = BB8Pool<AsyncDieselConnectionManager<AsyncPgConnection>>;
pub type Connection<'a> = PooledConnection<'a, AsyncDieselConnectionManager<AsyncPgConnection>>;

pub fn old_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).unwrap()
}

pub fn run_migrations(db: &mut PgConnection) {
    println!("Running migrations...");
    let mut harness = HarnessWithOutput::new(db, stdout());
    harness.run_pending_migrations(MIGRATIONS).unwrap();
    println!("Migrations complete.");
}

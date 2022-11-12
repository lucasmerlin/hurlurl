extern crate dotenv;

use diesel::prelude::*;

use diesel_async::{AsyncPgConnection};
use diesel_migrations::{
    embed_migrations, EmbeddedMigrations, HarnessWithOutput, MigrationHarness,
};
use dotenv::dotenv;
use native_tls::TlsConnector;
use postgres_native_tls::MakeTlsConnector;
use std::env;
use std::io::stdout;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

pub async fn db() -> AsyncPgConnection {
    dotenv().ok();

    let connector = TlsConnector::builder()
        .danger_accept_invalid_certs(true)
        .build()
        .unwrap();
    let connector = MakeTlsConnector::new(connector);

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let (client, connection) = tokio_postgres::connect(database_url.as_str(), connector)
        .await
        .expect("Error connecting to database");

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    let connection = AsyncPgConnection::try_from(client)
        .await
        .expect("Error connecting to database");

    connection
}

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

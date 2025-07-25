use diesel::prelude::*;

use diesel_async::AsyncPgConnection;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

use bb8::{ErrorSink, Pool as BB8Pool, PooledConnection};
use diesel_async::async_connection_wrapper::AsyncConnectionWrapper;
use diesel_async::pooled_connection::{AsyncDieselConnectionManager, ManagerConfig};
use futures_util::future::BoxFuture;
use futures_util::FutureExt;
use rustls::ClientConfig;
use rustls_platform_verifier::ConfigVerifierExt;
use std::fmt::Debug;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

pub type Pool = BB8Pool<AsyncDieselConnectionManager<AsyncPgConnection>>;
pub type Connection<'a> = PooledConnection<'a, AsyncDieselConnectionManager<AsyncPgConnection>>;

fn tls_establish_connection(config: &str) -> BoxFuture<'_, ConnectionResult<AsyncPgConnection>> {
    let fut = async {
        // We first set up the way we want rustls to work.
        let rustls_config = ClientConfig::with_platform_verifier();
        let tls = tokio_postgres_rustls::MakeRustlsConnect::new(rustls_config);
        let (client, conn) = tokio_postgres::connect(config, tls)
            .await
            .map_err(|e| ConnectionError::BadConnection(e.to_string()))?;

        AsyncPgConnection::try_from_client_and_connection(client, conn).await
    };
    fut.boxed()
}

pub async fn connect_and_migrate(url: &str) -> anyhow::Result<Pool> {
    let tls_config = rustls::ClientConfig::builder()
        .with_root_certificates(rustls::RootCertStore::empty())
        .with_no_client_auth();
    let tls = tokio_postgres_rustls::MakeRustlsConnect::new(tls_config);

    let mut manager_config = ManagerConfig::default();
    manager_config.custom_setup = Box::new(tls_establish_connection);

    let manager = AsyncDieselConnectionManager::new_with_config(url.to_owned(), manager_config);

    #[derive(Debug)]
    struct HurlurlErrorSink;

    impl<E: Debug> ErrorSink<E> for HurlurlErrorSink {
        fn sink(&self, error: E) {
            tracing::error!("Error in connection pool: {:?}", error);
        }

        fn boxed_clone(&self) -> Box<dyn ErrorSink<E>> {
            Box::new(HurlurlErrorSink)
        }
    }

    let pool = Pool::builder()
        .error_sink(Box::new(HurlurlErrorSink))
        .build(manager)
        .await
        .unwrap();

    {
        let pool_clone = pool.clone();
        let mut async_wrapper: AsyncConnectionWrapper<PooledConnection<_>> =
            AsyncConnectionWrapper::from(pool_clone.get_owned().await?);

        tokio::task::spawn_blocking(move || {
            tracing::info!("Running migrations...");
            async_wrapper.run_pending_migrations(MIGRATIONS).unwrap();
            tracing::info!("Migrations complete.");
        })
        .await?;
    }

    Ok(pool)
}

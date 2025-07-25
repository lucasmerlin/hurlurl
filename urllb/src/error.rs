use axum::http::StatusCode;
use diesel_async::pooled_connection::PoolError;
use tracing::error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Database error: {0}")]
    DieselError(#[from] diesel::result::Error),
    #[error("Stripe Error: {0}")]
    StripeError(#[from] stripe::StripeError),
    #[error("Pool error: {0}")]
    PoolError(#[from] bb8::RunError<PoolError>),
}

impl From<Error> for StatusCode {
    fn from(err: Error) -> Self {
        error!("Internal server error: {:?}", err);
        StatusCode::INTERNAL_SERVER_ERROR
    }
}

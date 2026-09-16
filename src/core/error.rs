use actix_web::{HttpResponse, ResponseError, body::BoxBody, http::StatusCode};
use config::ConfigError;
use diesel::result::Error as DieselError;
use r2d2::Error as ConnectionPoolError;
use redis::RedisError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ApiError {
    //
    // Infrastructure errors
    //
    #[error("Configuration error: {0}")]
    Config(#[from] ConfigError),

    #[error("Connection pool error: {0}")]
    ConnectionPool(#[from] ConnectionPoolError),

    #[error("Database error: {0}")]
    Database(#[from] DieselError),

    #[error("Redis error: {0}")]
    Redis(#[from] RedisError),

    #[error("Telemetry error: {0}")]
    Telemetry(String),

    //
    // HTTP errors
    //
    #[error("{0}")]
    BadRequest(String),

    #[error("{0}")]
    NotFound(String),

    #[error("Unauthorized access")]
    Unauthorized,

    #[error("Forbidden access")]
    Forbidden,

    #[error("Internal server error")]
    InternalServerError,
}

pub type Result<T> = std::result::Result<T, ApiError>;

impl ResponseError for ApiError {
    fn status_code(&self) -> StatusCode {
        match self {
            Self::BadRequest(_) => StatusCode::BAD_REQUEST,
            Self::NotFound(_) => StatusCode::NOT_FOUND,
            Self::Unauthorized => StatusCode::UNAUTHORIZED,
            Self::Forbidden => StatusCode::FORBIDDEN,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse<BoxBody> {
        let status_code = self.status_code();
        let payload = serde_json::json!({
            "message": self.to_string()
        });

        HttpResponse::build(status_code).json(payload)
    }
}

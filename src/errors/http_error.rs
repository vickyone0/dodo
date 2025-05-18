
use actix_web::{http::StatusCode, HttpResponse, ResponseError};
use serde_json::json;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum HttpError {
    #[error("Bad request: {0}")]
    BadRequest(String),
    
    #[error("Unauthorized: {0}")]
    Unauthorized(String),
    
    #[error("Internal server error")]
    InternalServerError,
    
    #[error("Database error")]
    DatabaseError(#[from] sqlx::Error),
    
    #[error("Validation error: {0}")]
    ValidationError(#[from] validator::ValidationErrors),
    
    #[error("JWT error")]
    JwtError(#[from] jsonwebtoken::errors::Error),
    
    #[error("Argon2 error")]
    Argon2Error(#[from] argon2::Error),
}

impl ResponseError for HttpError {
    fn status_code(&self) -> StatusCode {
        match self {
            HttpError::BadRequest(_) => StatusCode::BAD_REQUEST,
            HttpError::Unauthorized(_) => StatusCode::UNAUTHORIZED,
            HttpError::ValidationError(_) => StatusCode::BAD_REQUEST,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse {
        let status_code = self.status_code();
        let error_message = self.to_string();
        
        HttpResponse::build(status_code)
            .json(json!({ "error": error_message }))
    }
}
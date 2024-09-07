use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;

#[derive(Debug)]
pub struct ErrorResponse {
    pub error: String,
    pub status_code: StatusCode,
}

impl IntoResponse for ErrorResponse {
    fn into_response(self) -> Response {
        (
            self.status_code,
            Json(json!(
                {
                    "error": self.error
                }
            )),
        )
            .into_response()
    }
}

pub trait IntoErrorResponse {
    fn error(&self) -> ErrorResponse;
}

pub enum APIError {
    InvalidCategory(String),
    ItemAlreadyExists(String),
    AddingItemError(sqlx::Error),
    ItemNotFound(i32),
}

impl IntoErrorResponse for APIError {
    fn error(&self) -> ErrorResponse {
        match self {
            Self::InvalidCategory(category) => ErrorResponse {
                error: format!("Invalid category: {}", category),
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
            },
            Self::ItemAlreadyExists(name) => ErrorResponse {
                error: format!("Item is already exists: {}", name),
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
            },
            Self::AddingItemError(err) => ErrorResponse {
                error: format!("Failed to add item: {:?}", err),
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
            },
            Self::ItemNotFound(id) => ErrorResponse {
                error: format!("Item not found: {}", id),
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
            },
        }
    }
}

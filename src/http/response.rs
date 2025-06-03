use axum::{
    Json,
    http::StatusCode,
    response::IntoResponse,
    response::Response,
};
use serde::Serialize;

#[derive(Serialize)]
pub struct ErrorResponse {
    error: String,
}

pub struct AppError(anyhow::Error);

impl From<String> for AppError {
    fn from(err: String) -> Self {
        AppError(anyhow::anyhow!(err))
    }
}

impl From<&str> for AppError {
    fn from(err: &str) -> Self {
        AppError(anyhow::anyhow!(err.to_string()))
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                error: self.0.to_string(),
            }),
        )
            .into_response()
    }
}

pub struct AppJson<T>(pub T);

impl<T> IntoResponse for AppJson<T>
where
    axum::Json<T>: IntoResponse,
{
    fn into_response(self) -> Response {
        axum::Json(self.0).into_response()
    }
} 
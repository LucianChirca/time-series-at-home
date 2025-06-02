use axum::{
    Json,
    http::StatusCode,
};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct PostData {
    message: String,
}

pub async fn handle_post_data(Json(payload): Json<PostData>) -> (StatusCode, Json<PostData>) {
    println!("Received POST request with message: {}", payload.message);
    (StatusCode::OK, Json(payload))
}
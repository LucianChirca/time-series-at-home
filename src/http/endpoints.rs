use axum::{
    Json,
    http::StatusCode,
};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct PostData {
    value: f64,
    timestamp: i64,
}

pub async fn handle_post_data(Json(payload): Json<PostData>) -> (StatusCode, Json<PostData>) {
    println!("Received POST request with value: {}", payload.value);
    (StatusCode::OK, Json(payload))
}
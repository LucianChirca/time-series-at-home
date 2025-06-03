use axum::{
    Json,
    http::StatusCode,
};
use serde::{Deserialize, Serialize};
use crate::parquet::{ParquetConfig, writer::ParquetWriter};

#[derive(Deserialize, Serialize)]
pub struct PostData {
    value: f64,
    timestamp: i64,
}

pub async fn handle_post_data(Json(payload): Json<PostData>) -> (StatusCode, Json<PostData>) {
    println!("Received POST request with value: {}", payload.value);
    
    let config = ParquetConfig { 
        file_path: format!("files/data_{}.parquet", payload.timestamp)
    };
    let writer = ParquetWriter::new(config);
    
    if let Err(e) = writer.write(payload.value, payload.timestamp) {
        println!("Error writing to parquet: {}", e);
        return (StatusCode::INTERNAL_SERVER_ERROR, Json(payload));
    }
    
    (StatusCode::OK, Json(payload))
}
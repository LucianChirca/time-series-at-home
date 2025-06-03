use axum::{
    Json,
    http::StatusCode,
};
use serde::{Deserialize, Serialize};
use crate::parquet::{ParquetConfig, writer::ParquetWriter, reader::ParquetReader, reader::TimeSeriesData};

#[derive(Deserialize, Serialize)]
pub struct PostData {
    value: f64,
    timestamp: i64,
}

pub async fn handle_post_data(Json(payload): Json<PostData>) -> (StatusCode, Json<PostData>) {
    let config = ParquetConfig { 
        file_path: format!("files/data.parquet")
    };
    let writer = ParquetWriter::new(config);
    
    if let Err(e) = writer.write(payload.value, payload.timestamp) {
        println!("Error writing to parquet: {}", e);
        return (StatusCode::INTERNAL_SERVER_ERROR, Json(payload));
    }
    
    (StatusCode::OK, Json(payload))
}

pub async fn handle_get_data() -> (StatusCode, Json<Vec<TimeSeriesData>>) {
    let config = ParquetConfig {
        file_path: format!("files/data.parquet")
    };
    let reader = ParquetReader::new(config);
    
    match reader.read() {
        Ok(data) => {
            (StatusCode::OK, Json(data))
        },
        Err(e) => {
            println!("Error reading from parquet: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(Vec::new()))
        }
    }
}


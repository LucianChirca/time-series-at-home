use axum::{
    Json,
};
use serde::{Deserialize, Serialize};
use crate::parquet::{ParquetConfig, writer::ParquetWriter, reader::ParquetReader, reader::TimeSeriesData};
use super::response::{AppError, AppJson};

#[derive(Deserialize, Serialize)]
pub struct PostData {
    value: f64,
    timestamp: i64,
}

pub async fn handle_post_data(Json(payload): Json<PostData>) -> Result<AppJson<PostData>, AppError> {
    let config = ParquetConfig { 
        file_path: format!("files/data.parquet")
    };
    let writer = ParquetWriter::new(config);
    
    if let Err(e) = writer.write(payload.value, payload.timestamp) {
        println!("Error writing to parquet: {}", e);
        return Err(AppError::from(format!("Error writing to parquet: {}", e)));
    }
    
    Ok(AppJson(payload))
}

pub async fn handle_get_data() -> Result<AppJson<Vec<TimeSeriesData>>, AppError> {
    let config = ParquetConfig {
        file_path: format!("files/data.parquet")
    };
    let reader = ParquetReader::new(config);
    
    match reader.read() {
        Ok(data) => {
            Ok(AppJson(data))
        },
        Err(e) => {
            Err(AppError::from(format!("Error reading from parquet: {}", e)))
        }
    }
}


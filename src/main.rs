mod parquet;

use axum::{
    routing::post,
    Router,
    Json,
    http::StatusCode,
};
use clap::{Parser, Subcommand};
use parquet::{writer::ParquetWriter, ParquetConfig};
use std::sync::Arc;
use arrow::array::Int64Array;
use serde::{Deserialize, Serialize};

#[derive(Parser, Debug)]
#[command(about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    GenerateParquet {
        file: String,
    },
    StartServer {}
}

#[derive(Deserialize, Serialize)]
struct PostData {
    message: String,
}

async fn handle_post(Json(payload): Json<PostData>) -> (StatusCode, Json<PostData>) {
    println!("Received POST request with message: {}", payload.message);
    (StatusCode::OK, Json(payload))
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    match args.command {
        Commands::GenerateParquet { file } => {
            let config = ParquetConfig { file_path: file };
            let writer = ParquetWriter::new(config);

            let data = Arc::new(Int64Array::from_value(1, 10));
            writer.write(data).unwrap();
        }
        Commands::StartServer {} => {
            println!("Starting server on http://localhost:3000");
            
            let app = Router::new()
                .route("/", post(handle_post));

            let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
            axum::serve(listener, app).await.unwrap();
        }
    }
}

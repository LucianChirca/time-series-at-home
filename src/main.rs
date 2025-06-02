mod parquet;
mod http;

use clap::{Parser, Subcommand};
use parquet::{writer::ParquetWriter, ParquetConfig};
use std::sync::Arc;
use arrow::array::Int64Array;
use http::server::start_server;

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
            start_server().await;
        }
    }
}

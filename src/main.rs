mod parquet;
mod http;

use clap::{Parser, Subcommand};
use parquet::{writer::ParquetWriter, ParquetConfig};
use std::sync::Arc;
use arrow::array::Int64Array;
use http::server::start_server;
use http::ServerConfig;

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

            writer.write(1.0, 1717334400000).unwrap();
        }
        Commands::StartServer {} => {
            let config = ServerConfig::new("127.0.0.1".to_string(), 3000);
            start_server(config).await;
        }
    }
}

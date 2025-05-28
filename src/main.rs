mod parquet;

use clap::{Parser, Subcommand};
use parquet::{generator::ParquetGenerator, ParquetConfig};

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
}

fn main() {
    let args = Args::parse();

    match args.command {
        Commands::GenerateParquet { file } => {
            let config = ParquetConfig { file_path: file };
            let generator = ParquetGenerator::new(config);
            generator.generate().unwrap();
        }
    }
}

use clap::{Parser, Subcommand};

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
            println!("TODO: Generate parquet file at {}", file);
        }
    }
}

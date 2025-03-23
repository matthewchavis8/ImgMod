
mod args;
mod chunk;
mod chunk_type;
mod commands;
mod png;

use args::CliArgs;
use args::Commands;
use clap::Parser;
use commands::{decode, encode, remove, print_chunks};

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

fn main() {
    let cli = CliArgs::parse();

    match cli.command {
        Commands::Encode(args) => {
            if let Err(e) = encode(&args) {
                eprintln!("Encode error: {:?}", e);
            }
        }
        Commands::Decode(args) => {
            if let Err(e) = decode(&args) {
                eprintln!("Decode error: {:?}", e);
            }
        }
        Commands::Remove(args) => {
            if let Err(e) = remove(&args) {
                eprintln!("Remove error: {:?}", e);
            }
        }
        Commands::Print(args) => {
            if let Err(e) = print_chunks(&args) {
                eprintln!("Print error: {:?}", e);
            }
        }
    }
}
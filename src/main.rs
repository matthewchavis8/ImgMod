
mod args;
mod png;

use crate::args::
args::{CliArgs, Commands};
use crate::args::commands::{decode, encode, remove, print_chunks};
use clap::Parser;

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
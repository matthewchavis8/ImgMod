
mod args;
mod png;

use crate::args::
args::{CliArgs, Commands, ManageCommands};

use crate::args::commands::{decode, encode, remove, print_chunks, delete_file};
use args::commands::download_file;
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
        Commands::Manage(manage_args) => {

            match manage_args.manage_command {
                ManageCommands::Delete(args) => {
                    if let Err(e) = delete_file(&args) {
                        eprintln!("Print error: {:?}", e);
                    }
                }
                ManageCommands::Download(args) => {
                    if let Err(e) = download_file(&args) {
                        eprintln!("Print error: {:?}", e);
                    }
                }
                _ => {}
            }
        }
    }
}
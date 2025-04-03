
mod img_cli;
mod png;

use clap::Parser;
use imgmod::img_cli::args::CliArgs;

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

fn main() {
    let cli = CliArgs::parse();
    
    if let Err(e) = imgmod::run(cli) {
        eprintln!("Application error: {:?}", e);
    }
}

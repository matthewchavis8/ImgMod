pub mod png;
pub mod img_cli;

use crate::img_cli::args::{CliArgs, Commands, ManageCommands};
use crate::img_cli::commands::{decode, encode, remove, print_chunks, delete_file, download_file, convert_file};

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

pub fn run(cli: CliArgs) -> Result<()> {
       match cli.command {
        Commands::Encode(args) => encode(&args),
        Commands::Decode(args) => decode(&args),
        Commands::Remove(args) => remove(&args),
        Commands::Print(args) => print_chunks(&args),

        Commands::Manage(manage_args) => {
            match manage_args.manage_command {
                ManageCommands::Delete(args) => delete_file(&args),
                ManageCommands::Download(args) => download_file(&args),
                ManageCommands::Convert(args) => convert_file(&args),
            }
        }
    }
}

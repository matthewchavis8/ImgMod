use std::path::PathBuf;

use clap::{Args, Parser, Subcommand};

#[derive(Debug, Parser)]
pub struct CliArgs {
    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    Encode(EncodeArgs),
    Decode(DecodeArgs),
    Remove(RemoveArgs),
    Print(PrintArgs),
    Manage(ManageArgs)
}

#[derive(Debug, Clone, Args)]
pub struct EncodeArgs {
    pub file_path: PathBuf,
    pub chunk_type: String,
    pub message: String,
    pub output_file: Option<PathBuf>
}

#[derive(Debug, Clone, Args)]
pub struct DecodeArgs {
    pub file_path: PathBuf,
    pub chunk_type: String 
}

#[derive(Debug, Clone, Args)]
pub struct RemoveArgs {
    pub file_path: PathBuf,
    pub chunk_type: String
}

#[derive(Debug, Clone, Args)]
pub struct PrintArgs {
    pub file_path: PathBuf
}

/*MANAGE ARGS*/
#[derive(Debug, Parser)]
pub struct ManageArgs {
    #[clap(subcommand)]
    pub manage_command: ManageCommands,

}

#[derive(Debug, Subcommand)]
pub enum ManageCommands {
    Delete(DeleteArgs),
    Download(DownloadFromInternetArgs),
    Convert(ConvertArgs),
}

#[derive(Debug, Clone, Args)]
pub struct DeleteArgs {
    pub file_path: PathBuf
}

#[derive(Debug, Clone, Args)]
pub struct DownloadFromInternetArgs {
    pub url: String,
    pub output_file_name: PathBuf
}

#[derive(Debug, Clone, Args)]
pub struct ConvertArgs {
    #[arg(short = 'p')]
    pub convert_to_png: bool,

    #[arg(short = 'j')]
    pub convert_to_jpg: bool,

    #[arg(short = 't')]
    pub convert_to_tiff: bool,

    #[arg(short = 'w')]
    pub convert_to_webp: bool,

    pub input_path: PathBuf,
}

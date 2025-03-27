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
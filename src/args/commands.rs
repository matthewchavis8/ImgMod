use std::{fmt, fs};

use crate::args::args::
{DecodeArgs, 
EncodeArgs, 
PrintArgs, 
RemoveArgs};
use crate::png::image::{Png, PngError};
use crate::png::chunk::Chunk;

use super::args::{DeleteArgs, DownloadFromInternetArgs};

#[derive(Debug)]
pub enum CommandError {
    DownloadError,
    DeleteFileError,
    ConversionError,
}
impl fmt::Display for CommandError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CommandError::DeleteFileError => write!(f, "File does not exist cannot display"),
            CommandError::ConversionError => write!(f, "Failed to convert file"),
            CommandError::DownloadError => write!(f, "Failed to download file from the internet"),
        }
    }
}

impl std::error::Error for CommandError {}

pub fn encode(args: &EncodeArgs) -> Result<(),  Box<dyn std::error::Error>> {
    let mut png = Png::from_file(&args.file_path)?;
    let chunk = Chunk::from_strings(&args.chunk_type, &args.message)?;
    png.append_chunk(chunk);

    if let Some(output_file) = &args.output_file {
        png.write_file(output_file)?
    } else {
        png.write_file(&args.file_path)?
    }

    Ok(())
}

pub fn decode(args: &DecodeArgs) -> Result<(), Box<dyn std::error::Error>> {
    let png = Png::from_file(&args.file_path)?;
    
    match png.chunk_by_type(&args.chunk_type) {
        Some(chunk) => {
            println!("msg: {}", chunk.data_as_string()?);
            Ok(())
        }
        None => Err(Box::new(PngError::InvalidChunk))
    }
}

pub fn remove(args: &RemoveArgs) -> Result<(), Box<dyn std::error::Error>> {
    let mut png = Png::from_file(&args.file_path)?;
    png.remove_chunk(&args.chunk_type)?;
    
    png.write_file(&args.file_path)?;

    Ok(())
}

pub fn print_chunks(args: &PrintArgs) -> Result<(), Box<dyn std::error::Error>> {
    let png = Png::from_file(&args.file_path)?;
    println!(
        "File: {}, Size: {}",
        &args.file_path.display(),
        png.as_bytes().len()
    );

    for (i, chunk) in png.chunks().iter().enumerate() {
        println!(
            "  chunk#{}{{ chunk_type: {}, data_length: {}}}",
            i,
            chunk.chunk_type(),
            chunk.length(),
        );
    }
    Ok(())
}

pub fn delete_file(args: &DeleteArgs) -> Result<(), Box<dyn std::error::Error>> {
    let file = &args.file_path;

    if file.exists() {
        fs::remove_file(file)?;
        println!("Deleted: {:?}", file);
        Ok(())
    } else {
        println!("No file at path: {:?}", file);
        Err(Box::new(CommandError::DeleteFileError))
    }

}

pub fn download_file(args: &DownloadFromInternetArgs) -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}
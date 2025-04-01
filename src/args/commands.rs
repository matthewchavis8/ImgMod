use std::fs::File;
use std::path::Path;
use std::{fmt, fs};
use std::io::copy;
use reqwest::blocking::get;

use crate::args::args::
{DecodeArgs, 
EncodeArgs, 
PrintArgs, 
RemoveArgs};
use crate::png::image::{Png, PngError};
use crate::png::chunk::Chunk;
#[allow(deprecated)]
use image::io::Reader as ImageReader;
use image::ImageFormat;


use super::args::{ConvertArgs, DeleteArgs, DownloadFromInternetArgs};
extern crate reqwest;

#[derive(Debug)]
pub enum CommandError {
    DownloadError,
    DeleteFileError,
    ConversionError,
    FailedToFindURL,
}
impl fmt::Display for CommandError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CommandError::DeleteFileError => write!(f, "File does not exist cannot display"),
            CommandError::ConversionError => write!(f, "Failed to convert file"),
            CommandError::DownloadError => write!(f, "Failed to download file from the internet"),
            CommandError::FailedToFindURL => write!(f, "Failed to find URL from the internet"),
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

pub fn delete_file(args: &DeleteArgs) -> Result<(), CommandError> {
    let file = &args.file_path;

    if file.exists() {
        fs::remove_file(file).map_err(|_| CommandError::DeleteFileError)?;
        println!("Deleted: {:?}", file);
        Ok(())
    } else {
        println!("No file at path: {:?}", file);
        Err(CommandError::DeleteFileError)
    }

}

pub fn download_file(args: &DownloadFromInternetArgs) -> Result<(), CommandError> {
    let res = get(args.url.clone())
        .map_err(|_| CommandError::DownloadError)?;

    if !res.status().is_success() {
        return Err(CommandError::FailedToFindURL);
    }

    let images_dir = Path::new("images");
    let file_path = images_dir.join(&args.output_file_name);

    let mut output_file = File::create(&file_path)
        .map_err(|_| CommandError::DownloadError)?;

    let mut image = res;
    copy(&mut image, &mut output_file)
        .map_err(|_| CommandError::DownloadError)?;

    println!("Download file to: {:?}", file_path);
    Ok(())
}

pub fn convert_file(args: &ConvertArgs) -> Result<(), CommandError> {
    let img = ImageReader::open(&args.input_path)
        .map_err(|_| CommandError::ConversionError)?
        .decode().map_err(|_| CommandError::ConversionError)?;
    
    let mut output_path = args.input_path.clone();
    
    let format: Option<image::ImageFormat> = if args.convert_to_jpg {
        output_path.set_extension("jpeg");
        Some(image::ImageFormat::Jpeg)
    } else if args.convert_to_png {
        output_path.set_extension("png");
        Some(image::ImageFormat::Png)
    } else if args.convert_to_tiff {
        output_path.set_extension("tiff");
        Some(image::ImageFormat::Tiff)
    } else if args.convert_to_webp {
        output_path.set_extension("webp");
        Some(image::ImageFormat::WebP)
    } else {
        None
    };
    
    match format {
        Some(fmt) => {
            img.save_with_format(&output_path, fmt)
                .map_err(|_| CommandError::ConversionError)?;
            println!("Image converted and saved to {:?}", output_path);
            Ok(())
        }
        None => { 
            println!("No conversion format selected");
            Err(CommandError::ConversionError)
        }
    }
}

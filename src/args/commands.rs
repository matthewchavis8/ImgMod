use crate::args::args::{DecodeArgs, EncodeArgs, PrintArgs, RemoveArgs};
use crate::png::image::{Png, PngError};
use crate::png::chunk::Chunk;

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
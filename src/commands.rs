use crate::args::{DecodeArgs, EncodeArgs, PrintArgs, RemoveArgs};
use crate::png::Png;
use crate::chunk::Chunk;

pub enum CommandError{
    InvalidEncode,
    InvalidDecode,
    InvalidRemove
}

pub fn encode(args: &EncodeArgs) -> Result<(),  CommandError> {
    let mut png = Png::from_file(&args.file_path).unwrap();
    let chunk = Chunk::from_strings(&args.chunk_type, &args.message).unwrap();
    png.append_chunk(chunk);

    if let Some(output_file) = &args.output_file {
        png.write_file(output_file).map_err(|_|CommandError::InvalidEncode)
    } else {
        png.write_file(&args.file_path).map_err(|_| CommandError::InvalidEncode)
    }
}

pub fn decode(args: &DecodeArgs) -> Result<(), CommandError> {
    let png = Png::from_file(&args.file_path).unwrap();
    
    if let Some(chunk) = png.chunk_by_type(&args.chunk_type) {
        println!("msg: {}", chunk.data_as_string().unwrap());
        Ok(())
    } else {
        Err(CommandError::InvalidDecode)
    }
}

pub fn remove(args: &RemoveArgs) -> Result<(), CommandError> {
    let mut png = Png::from_file(&args.file_path).unwrap();
    png.remove_chunk(&args.chunk_type).unwrap();
    
    png.write_file(&args.file_path).map_err(|_| CommandError::InvalidRemove)
}

pub fn print_chunks(args: &PrintArgs) -> Result<(), CommandError> {
    let png = Png::from_file(&args.file_path).unwrap();
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
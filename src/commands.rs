use crate::args::EncodeArgs;
use crate::png::Png;
use crate::chunk::Chunk;

pub enum CommandError{
    InvalidEncode
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

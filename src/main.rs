pub mod args;
pub mod chunk;
pub mod chunk_type;
pub mod commands;
pub mod png;

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

// fn main() -> Result<()> {}
fn main() {}
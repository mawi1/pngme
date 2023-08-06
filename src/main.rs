#![allow(dead_code)]

mod chunk;
mod chunk_type;
mod cli;
mod png;

use std::path::PathBuf;
use std::{fs, str::FromStr};

use clap::Parser;

use cli::{Cli, DecodeArgs, EncodeArgs, PrintArgs, RemoveArgs};
use png::Png;

pub type Error = anyhow::Error;
pub type Result<T> = std::result::Result<T, Error>;

fn main() -> Result<()> {
    let cli = Cli::parse();

    fn png_from_path(path: &PathBuf) -> Result<Png> {
        let data = fs::read(path)?;
        Png::try_from(&data[..])
    }

    match cli.command {
        cli::Commands::Encode(EncodeArgs {
            path,
            chunk_type,
            message,
            output,
        }) => {
            let mut png = png_from_path(&path)?;

            let chunk_type = chunk_type::ChunkType::from_str(chunk_type.as_str())?;
            let chunk = chunk::Chunk::new(chunk_type, message.as_bytes().to_vec());
            png.append_chunk(chunk);

            fs::write(output.unwrap_or(path), png.as_bytes())?;
        }
        cli::Commands::Decode(DecodeArgs { path, chunk_type }) => {
            let png = png_from_path(&path)?;

            if let Some(c) = png.chunk_by_type(&chunk_type) {
                println!("{}", c.data_as_string()?);
            }
        }
        cli::Commands::Remove(RemoveArgs { path, chunk_type }) => {
            let mut png = png_from_path(&path)?;

            png.remove_chunk(&chunk_type)?;

            fs::write(path, png.as_bytes())?;
        }
        cli::Commands::Print(PrintArgs { path }) => {
            let png = png_from_path(&path)?;
            println!("{}", png);
        }
    }
    Ok(())
}

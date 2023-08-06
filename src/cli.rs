use std::path::PathBuf;

use clap::{Args, Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(name = "pngme")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    Encode(EncodeArgs),
    Decode(DecodeArgs),
    Remove(RemoveArgs),
    Print(PrintArgs),
}

#[derive(Debug, Args)]
pub struct EncodeArgs {
    pub path: PathBuf,
    pub chunk_type: String,
    pub message: String,
    pub output: Option<PathBuf>,
}

#[derive(Debug, Args)]
pub struct DecodeArgs {
    pub path: PathBuf,
    pub chunk_type: String,
}

#[derive(Debug, Args)]
pub struct RemoveArgs {
    pub path: PathBuf,
    pub chunk_type: String,
}

#[derive(Debug, Args)]
pub struct PrintArgs {
    pub path: PathBuf,
}

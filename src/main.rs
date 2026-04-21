mod utils;
mod encoder;
mod decoder;

use anyhow::Result;
use clap::{Parser, Subcommand};
use encoder::Encoder;
use decoder::Decoder;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Encode a file into a video
    Encode {
        /// Input file path
        #[arg(short, long)]
        input: String,

        /// Output video path (e.g., output.mkv)
        #[arg(short, long)]
        output: String,

        /// Block size (bigger = less storage density, more robust against compression). Default 4.
        #[arg(short, long, default_value_t = 4)]
        block_size: usize,

        /// Data shards for Reed-Solomon (Default 10)
        #[arg(long, default_value_t = 10)]
        data_shards: usize,

        /// Parity shards for Reed-Solomon (Default 2)
        #[arg(long, default_value_t = 2)]
        parity_shards: usize,
        
        /// Ffmpeg codec to use (Default "ffv1" for lossless, or "libx264")
        #[arg(long, default_value = "ffv1")]
        codec: String,
    },
    /// Decode a video back to file
    Decode {
        /// Input video path
        #[arg(short, long)]
        input: String,

        /// Output file path
        #[arg(short, long)]
        output: String,
    },
}
use crate::utils::{FileHeader, FRAME_RATE, PIXEL_FORMAT, VIDEO_HEIGHT, VIDEO_WIDTH};
use anyhow::{Context, Result};
use indicatif::{ProgressBar, ProgressStyle};
use rayon::prelude::*
use reed_solomon_erasure::galois_8::ReedSolomon;
use sha2::{Digest, Sha256};
use std::io::{Read, Write};
use std::process::{Command, Stdio};

/// Handles the encoding process of video files, including Reed-Solomon erasure coding.
pub struct Encoder {
    input_path: String,
    output_path: String,
    block_size: usize,
    data_shards: usize,
    parity_shards: usize,
    codec: String,
}

impl Encoder {
    /// Creates a new Encoder instance.
    ///
    /// # Arguments
    /// * `input_path` - The path to the input file to be encoded.
    /// * `output_path` - The path where the encoded output will be saved.
    /// * `block_size` - The block size used for video partitioning.
    /// * `data_shards` - The number of data shards to generate.
    /// * `parity_shards` - The number of parity shards to generate.
    /// * `codec` - The video codec to use (e.g., "libx264").
    pub fn new(
        input_path: String,
        output_path: String,
        block_size: usize,
        data_shards: usize,
        parity_shards: usize,
        codec: String,
    ) -> Self {
        Self {
            input_path,
            output_path,
            block_size,
            data_shards,
            parity_shards,
            codec,
        }
    }

    /// Runs the video encoding and erasure coding process.
    ///
    /// This method executes ffmpeg to read the input, encode it using the specified codec,
    /// and generate Reed-Solomon parity shards.
    pub fn run(&self) -> Result<()> {
        let mut file = std::fs::File::open(&self.input_path).context("Failed to open input file")?;
        let file_size = file.metadata()?.len();
        
        // Calculate frame capacity
        let cols = VIDEO_WIDTH / self.block_size;
        let rows = VIDEO_HEIGHT / self.block_size;
        let bits_per_frame = cols * rows;
        let bytes_per_frame = bits_per_frame / 8;

        println!("Video Resolution: {}x{}", VIDEO_WIDTH, VIDEO_HEIGHT);
        println!("Block Size: {}x{}", self.block_size, self.block_size);
        println!("Frame Capacity: {} bytes", bytes_per_frame);

        // Setup FFmpeg
        let mut child = Command::new("ffmpeg")
            .args([
                "-f", "rawvideo",
                "-pixel_format", PIXEL_FORMAT,
                "-video_size", &format!("{}{}", VIDEO_WIDTH, VIDEO_HEIGHT),
                "-framerate", &format!("{}", FRAME_RATE),
                "-i", "pipe:0",
                "-c:v", &self.codec,
                "-g", "1", // Keyframe every frame for robustness
                "-y",
                &self.output_path,
            ])
            .stdin(Stdio::piped())
            .stdout(Stdio::null())
            .stderr(Stdio::inherit())
            .spawn()
            .context("Failed to spawn ffmpeg")?;

        let mut ffmpeg_stdin = child.stdin.take().context("Failed to open ffmpeg stdin")?;

        // Initialize RS
        let rs = ReedSolomon::new(self.data_shards, self.parity_shards)
            .context("Failed to create ReedSolomon")?;

        // Calculate hashing and progress
        let mut hasher = Sha256::new();
        let pb = ProgressBar::new(file_size);
        pb.set_style(ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {bytes}/{total_bytes} ({eta})")?
            .progress_chars("#>-"));

        // Header
        let header = FileHeader::new(
            self.input_path.clone(),
            file_size,
            self.block_size as u32,
            "PENDING".to_string(), // Hash will be verified at end 
        );

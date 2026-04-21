use crate::utils::{FileHeader, PIXEL_FORMAT, VIDEO_HEIGHT, VIDEO_WIDTH};
use anyhow::{Context, Result};
use indicatif::{ProgressBar, ProgressStyle};
use rayon::prelude::*
use reed_solomon_erasure::galois_8::ReedSolomon;
use sha2::{Digest, Sha256};
use std::io::{Read, Write};
use std::process::{Command, Stdio};

/// Handles the decoding process of video files.
pub struct Decoder {
    input_path: String,
    output_path: String,
}

impl Decoder {
    /// Creates a new Decoder instance.
    ///
    /// # Arguments
    /// * `input_path` - The path to the input video file.
    /// * `output_path` - The path where the decoded output will be saved.
    pub fn new(input_path: String, output_path: String) -> Self {
        Self {
            input_path,
            output_path,
        }
    }

    /// Runs the video decoding process using ffmpeg.
    ///
    /// This method spawns ffmpeg to read the input video and processes it according
    /// to the defined parameters.
    pub fn run(&self) -> Result<()> {
        // Spawn FFmpeg to read video
        let mut child = Command::new("ffmpeg")
            .args([
                "-i", &self.input_path,
                "-f", "rawvideo",
                "-pix_fmt", PIXEL_FORMAT,
                "-",
            ])
            .stdout(Stdio::piped())
            .stderr(Stdio::null()) // Suppress logs
            .spawn()
            .context("Failed to spawn ffmpeg decodder")?;

        let mut ffmpeg_stdout = child.stdout.take().context("Failed to open ffmpeg stdout")?;

        // Buffer for one frame
        let frame_size = VIDEO_WIDTH * VIDEO_HEIGHT * 3;
        let mut frame_buffer = vec![0u8; frame_size];

        // 1. Read Header from first frame(s)
        // We know we wrote header in the first frame.
        if ffmpeg_stdout.read_exact(&mut frame_buffer).is_err() {
            return Err(anyhow::anyhow!("Failed to read header frame"));
        }

        // Decode Header Frame
        // We don't know block size yet, but the header was written with a specific block size?
        // Wait, if block_size is configurable, we can't extract the header unless we know the block size used for the header!
        // OR we can try to detect it.
        // For this MVP, let's assume the user passes the same --block-size or we fix the header block size.
        // The user prompt said: "Header ... custom header".
        // Let's assume we decode with a separate logic for the header.
        // Let's try to decode the first frame with various block sizes? No, that's slow/complex.
        // Let's look at `utils.rs`: `block_size` is IN the header.
        // So we must be able to read the header WITHOUT knowing the block size.
        // This implies Header should use a FIXED block size (e.g. 1 or 4) so we can always read it.
        // Implementation Detail: Encoder must use fixed block size for header.
        // Let's update this Logic.
        // F
... (truncated)
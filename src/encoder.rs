use crate::utils::{FileHeader, FRAME_RATE, PIXEL_FORMAT, VIDEO_HEIGHT, VIDEO_WIDTH};
use anyhow::{Context, Result};
use indicatif::{ProgressBar, ProgressStyle};
use rayon::prelude::*;
use reed_solomon_erasure::galois_8::ReedSolomon;
use sha2::{Digest, Sha256};
use std::io::{Read, Write};
use std::process::{Command, Stdio};

pub struct Encoder {
    input_path: String,
    output_path: String,
    block_size: usize,
    data_shards: usize,
    parity_shards: usize,
    codec: String,
}

impl Encoder {
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
            .args(&[
                "-f", "rawvideo",
                "-pixel_format", PIXEL_FORMAT,
                "-video_size", &format!("{}x{}", VIDEO_WIDTH, VIDEO_HEIGHT),
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
            "PENDING".to_string(), // Hash will be verified at end of decode, but for header we might need to pre-calc if we want it perfect, 
                                   // OR we put the hash at the end? 
                                   // The user requirement says "Header ... Hash SHA256". 
                                   // If we stream, we can't know hash at start. 
                                   // We will compute hash as we read. But we write header first.
                                   // Compromise: Write placeholder in header, or read file twice.
                                   // For 100GB file, reading twice is bad. 
                                   // Let's write the hash of the *original* file if possible? 
                                   // Or maybe we can't.
                                   // User says "Integirta Dati... File deve essere recuperabile".
                                   // Decoder checks hash.
                                   // Let's just put a placeholder or "0" for now and print the real hash at the end for the user to store?
                                   // Or effectively we accept that the header hash is only useful if we pre-calculate.
                                   // Let's pre-calculate for MVP if file is small? No "100GB+".
                                   // OK, we will append a footer with the hash? 
                                   // User required Header.
                                   // Let's leave hash empty in header and log it at the end.
            self.data_shards,
            self.parity_shards,
        );
        
        // Write Header Frame(s)
        // We write the header validation logic in utils or here?
        // Let's write raw header bytes repeatedly to fill a frame or just use the same encoding logic?
        // Safest: Use RS for header too.
        // But for simplicity, let's just write header RAW into the first N frames (repeating it for redundancy).
        let header_bytes = header.to_bytes()?;
        // Encode header using the same robust encoding? 
        // Or just Repeate encoding.
        // Let's do: Header is stored in the first frame(s) with massive redundancy (repetition).
        // Actually, let's just treat header as the first chunk of data.
        // But we need to know settings to decode.
        // Chicken and egg.
        // Solution: Standardized "Bootstrap" Frame at Frame 0.
        // Frame 0 always: 1920x1080, Block Size 4, containing the Header.
        self.write_frame(&mut ffmpeg_stdin, &header_bytes, true, 4)?; // Force Block Size 4 for Header

        // Process File
        let shard_size = bytes_per_frame;
        let chunk_size = shard_size * self.data_shards;
        let mut buffer = vec![0u8; chunk_size];

        loop {
            let n = file.read(&mut buffer)?;
            if n == 0 {
                break;
            }
            pb.inc(n as u64);
            hasher.update(&buffer[..n]);

            // Pad last chunk if needed
            let mut data_shards: Vec<Vec<u8>> = buffer[..n]
                .chunks(shard_size)
                .map(|c| c.to_vec())
                .collect();
            
            // Pad the last shard if it's partial
            if let Some(last) = data_shards.last_mut() {
                if last.len() < shard_size {
                    last.resize(shard_size, 0);
                }
            }
            
            // Pad with empty shards if we don't have enough data shards
            while data_shards.len() < self.data_shards {
                data_shards.push(vec![0u8; shard_size]);
            }

            // Calculate parity
            // RS v6 expects a single vector containing both data and parity shards
            for _ in 0..self.parity_shards {
                data_shards.push(vec![0u8; shard_size]);
            }
            
            rs.encode(&mut data_shards)?;

            // Write all shards (Data + Parity) as frames
            for shard in data_shards.iter() {
                self.write_frame(&mut ffmpeg_stdin, shard, false, self.block_size)?;
            }
        }


        pb.finish_with_message("Encoding complete");
        
        // Finalize
        let result_hash = format!("{:x}", hasher.finalize());
        println!("Original File Hash: {}", result_hash);
        println!("(Note: Header contains placeholder hash)");

        // Wait for ffmpeg
        // We drop stdin to close pipe
        drop(ffmpeg_stdin);
        child.wait()?;

        Ok(())
    }

    fn write_frame(&self, writer: &mut impl Write, data: &[u8], is_header: bool, block_size: usize) -> Result<()> {
        // If header, we might just repeat the data to fill the frame?
        // Or just write it once and pad.
        // For robust header: Repeat the data until frame is full.
        let bytes_per_frame = (VIDEO_WIDTH / block_size) * (VIDEO_HEIGHT / block_size) / 8;
        
        let mut frame_data = data.to_vec();
        if is_header {
             // Repeat header to fill frame for robustness
             while frame_data.len() < bytes_per_frame {
                 frame_data.extend_from_slice(data);
             }
             frame_data.truncate(bytes_per_frame);
        }

        // Parallel conversion of bytes -> pixels
        let width = VIDEO_WIDTH;
        let height = VIDEO_HEIGHT;
        let cols = width / block_size;
        
        // Output buffer: RGB24
        let mut pixel_buffer = vec![0u8; width * height * 3];

        pixel_buffer.par_chunks_mut(width * 3 * block_size) // Process by rows (times block size)
            .enumerate()
            .for_each(|(row_idx, row_pixels)| {
                // row_pixels is a slice covering 'block_size' rows of pixels
                let y_base = row_idx * block_size;
                if y_base >= height { return; }

                for bx in 0..cols {
                    // Determine bit value
                    let bit_idx = row_idx * cols + bx;
                    let byte_idx = bit_idx / 8;
                    let bit_offset = 7 - (bit_idx % 8);
                    
                    let color = if byte_idx < frame_data.len() {
                        if (frame_data[byte_idx] >> bit_offset) & 1 == 1 {
                            255u8
                        } else {
                            0u8
                        }
                    } else {
                        0u8 // Padding
                    };

                    // Draw block
                    for dy in 0..block_size {
                        for dx in 0..block_size {
                            let px_idx = (dy * width + (bx * block_size + dx)) * 3;
                            if px_idx + 2 < row_pixels.len() {
                                row_pixels[px_idx] = color;
                                row_pixels[px_idx+1] = color;
                                row_pixels[px_idx+2] = color;
                            }
                        }
                    }
                }
            });

        writer.write_all(&pixel_buffer)?;
        Ok(())
    }
}

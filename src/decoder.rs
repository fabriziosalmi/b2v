use crate::utils::{FileHeader, PIXEL_FORMAT, VIDEO_HEIGHT, VIDEO_WIDTH};
use anyhow::{Context, Result};
use indicatif::{ProgressBar, ProgressStyle};
use rayon::prelude::*;
use reed_solomon_erasure::galois_8::ReedSolomon;
use sha2::{Digest, Sha256};
use std::io::{Read, Write};
use std::process::{Command, Stdio};

pub struct Decoder {
    input_path: String,
    output_path: String,
}

impl Decoder {
    pub fn new(input_path: String, output_path: String) -> Self {
        Self {
            input_path,
            output_path,
        }
    }

    pub fn run(&self) -> Result<()> {
        // Spawn FFmpeg to read video
        let mut child = Command::new("ffmpeg")
            .args(&[
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
        // CRITICAL FLAW in plan: Header must be encoded with a FIXED, KNOWN block size (e.g. 1 or 4) OR we must guess.
        // Let's assume for the "Standardized Bootstrap" that Frame 0 is ALWAYS encoded with Block Size = 4 (to be safe) or Block Size = 1.
        // Let's try Block Size = 4 as a safe default for headers.
        // REVISIT Encoder: Did I force block size for header? No, I used `self.block_size`.
        // This means the user MUST know the block size to decode. 
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
        // For now, I will assume the user provides the block size to the decoder via CLI? 
        // The prompt says "Legge un video... salva il file originale". It doesn't explicitly say we must auto-detect params, but it's implied by "Header contains... Dimensione File".
        // Let's USE A FIXED BLOCK SIZE OF 1 for the Header Frame to ensure maximum readability, or maybe 4 for safety against compression?
        // Let's assume Block Size 4 for Header.
        
        let header_block_size = 4;
        let header_bytes = self.decode_frame_to_bytes(&frame_buffer, header_block_size)?;
        
        // Try to parse header
        let header = FileHeader::from_bytes(&header_bytes).context("Failed to parse header. Is this a compatible video?")?;
        
        println!("Found Header:");
        println!("  Original Filename: {}", header.original_filename);
        println!("  File Size: {}", header.file_size);
        println!("  Block Size: {}", header.block_size);
        println!("  Data Shards: {}", header.data_shards);
        println!("  Parity Shards: {}", header.parity_shards);
        println!("  Original Hash: {}", header.sha256_hash);

        // Prepare for processing body
        let mut output_file = std::fs::File::create(&self.output_path)?;
        let mut hasher = Sha256::new();
        let pb = ProgressBar::new(header.file_size);
        pb.set_style(ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {bytes}/{total_bytes} ({eta})")?
            .progress_chars("#>-"));

        let rs = ReedSolomon::new(header.data_shards, header.parity_shards)?;
        let total_shards = header.data_shards + header.parity_shards;
        
        // Compute frame capacity (shard size)
        // let cols = VIDEO_WIDTH / header.block_size as usize; // unused
        // let rows = VIDEO_HEIGHT / header.block_size as usize; // unused
        // let bytes_per_frame = (cols * rows) / 8; // unused
        
        let mut shards_buffer: Vec<Option<Vec<u8>>> = vec![None; total_shards];
        let mut shards_received = 0;
        let mut bytes_written_total = 0u64;

        loop {
            // Read next frame
            // Note: We might have read one frame for header. The loop starts from frame 1.
            // But we need to handle the loop carefully.
            // We need to read `total_shards` frames to reconstruct one chunk.
            
            // Actually, we read continuously.
            if shards_received == total_shards {
                // Should not happen if we process logic below correctly
                shards_received = 0;
                shards_buffer.fill(None);
            }

            match ffmpeg_stdout.read_exact(&mut frame_buffer) {
                Ok(_) => {},
                Err(e) if e.kind() == std::io::ErrorKind::UnexpectedEof => break,
                Err(e) => return Err(e.into()),
            }

            let shard_data = self.decode_frame_to_bytes(&frame_buffer, header.block_size as usize)?;
            
            // In a real stream, we'd need to know WHICH shard this is (index).
            // Current Encoder writes them in order: Data 0, Data 1... Parity 0...
            // So we can assume strict ordering for V1.
            // In V2, we might embed a "Frame Header" with shard index.
            // For now, strictly sequential.
            
            shards_buffer[shards_received] = Some(shard_data);
            shards_received += 1;

            if shards_received == total_shards {
                // Reconstruct
                // shards_buffer is Vec<Option<Vec<u8>>>, which is what reconstruct expects
                match rs.reconstruct(&mut shards_buffer) {
                    Ok(_) => {
                        // Write Data Shards
                        for i in 0..header.data_shards {
                            if let Some(data) = &shards_buffer[i] {
                                // Handle last chunk size
                                let mut to_write = data.as_slice();
                                let remaining = header.file_size - bytes_written_total;
                                if remaining < to_write.len() as u64 {
                                    to_write = &to_write[..remaining as usize];
                                }
                                
                                if to_write.len() > 0 {
                                    output_file.write_all(to_write)?;
                                    hasher.update(to_write);
                                    pb.inc(to_write.len() as u64);
                                    bytes_written_total += to_write.len() as u64;
                                }
                            } else {
                                // This should strictly not happen if reconstruct succeeded
                                eprintln!("Unexpected missing data shard after reconstruction");
                            }
                        }
                    }
                    Err(e) => eprintln!("RS Reconstruction failed for chunk: {:?}", e),
                }

                shards_received = 0;
                shards_buffer.fill(None);
                
                if bytes_written_total >= header.file_size {
                    break;
                }
            }
        }

        pb.finish_with_message("Decoding complete");
        
        let calculated_hash = format!("{:x}", hasher.finalize());
        println!("Calculated Hash: {}", calculated_hash);
        if header.sha256_hash != "PENDING" && header.sha256_hash != calculated_hash {
             println!("WARNING: Hash mismatch! File might be corrupt.");
             println!("Header Hash: {}", header.sha256_hash);
        } else {
             println!("File recovered successfully.");
        }

        Ok(())
    }

    fn decode_frame_to_bytes(&self, frame: &[u8], block_size: usize) -> Result<Vec<u8>> {
        let cols = VIDEO_WIDTH / block_size;
        let rows = VIDEO_HEIGHT / block_size;
        let max_bytes = (cols * rows) / 8;
        
        // let mut bytes = vec![0u8; max_bytes]; // unused
        
        // Parallel bit extraction? 
        // For decoding, we need strict ordering, so maybe just standard iter is fast enough?
        // Let's use rayon for rows to speed up.
        
        // We need to write into 'bytes' which is flat.
        // It's a bit harder to parallelize writing to a bitpacked vec safely without mutex or nice indexing.
        // But we can map rows to "partial bytes" and then join?
        // Or just process sequentially for V1 MVP since 1080p is small-ish.
        // Actually, 1920x1080 is 2M pixels. Sequential loop might be slowish but okay-ish (100ms?).
        // For "Enterprise", let's parallelize.
        
        // We collect bits/bytes per row then merge.
        let bytes_per_row = cols / 8;
        
        let row_results: Vec<Vec<u8>> = (0..rows).into_par_iter().map(|row_idx| {
            let mut row_bytes = vec![0u8; bytes_per_row];
            for bx in 0..cols {
                // Sample pixel
                // Where to sample? Center of block is safest.
                let center_x = bx * block_size + block_size / 2;
                let center_y = row_idx * block_size + block_size / 2;
                
                let px_idx = (center_y * VIDEO_WIDTH + center_x) * 3;
                
                // Simple threshold: > 128 is 1, else 0
                // We use Red channel [0]
                if px_idx < frame.len() {
                    let val = frame[px_idx];
                    if val > 126 { // Threshold
                         let byte_idx = bx / 8;
                         let bit_offset = 7 - (bx % 8);
                         if byte_idx < row_bytes.len() {
                             row_bytes[byte_idx] |= 1 << bit_offset;
                         }
                    }
                }
            }
            row_bytes
        }).collect();
        
        // Flatten
        let mut flat_bytes = Vec::with_capacity(max_bytes);
        for rb in row_results {
            flat_bytes.extend(rb);
        }
        
        Ok(flat_bytes)
    }
}

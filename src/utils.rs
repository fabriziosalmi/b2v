use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

// Constants
pub const MAGIC_NUMBER: u64 = 0x4554_524E_4C53_5452; // "ETRNLSTR" in hex
pub const HEADER_SIZE: usize = 1024; // Fixed header size
pub const VERSION: u32 = 1;

// Video Settings
pub const VIDEO_WIDTH: usize = 1920;
pub const VIDEO_HEIGHT: usize = 1080;
pub const FRAME_RATE: usize = 30;
pub const PIXEL_FORMAT: &str = "rgb24";

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FileHeader {
    pub magic: u64,
    pub version: u32,
    pub original_filename: String,
    pub file_size: u64,
    pub block_size: u32,
    pub sha256_hash: String,
    pub data_shards: usize,
    pub parity_shards: usize,
}

impl FileHeader {
    pub fn new(
        original_filename: String,
        file_size: u64,
        block_size: u32,
        sha256_hash: String,
        data_shards: usize,
        parity_shards: usize,
    ) -> Self {
        Self {
            magic: MAGIC_NUMBER,
            version: VERSION,
            original_filename,
            file_size,
            block_size,
            sha256_hash,
            data_shards,
            parity_shards,
        }
    }

    pub fn to_bytes(&self) -> Result<Vec<u8>> {
        let json = serde_json::to_string(self).context("Failed to serialize header")?;
        let bytes = json.as_bytes();
        if bytes.len() > HEADER_SIZE {
            return Err(anyhow::anyhow!("Header too large"));
        }
        let mut padded = vec![0u8; HEADER_SIZE];
        padded[..bytes.len()].copy_from_slice(bytes);
        Ok(padded)
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Self> {
        // Find the first null byte to determine end of JSON string
        let len = bytes.iter().position(|&x| x == 0).unwrap_or(bytes.len());
        let json_str = std::str::from_utf8(&bytes[..len]).context("Invalid UTF-8 in header")?;
        let header: FileHeader = serde_json::from_str(json_str).context("Failed to deserialize header")?;
        if header.magic != MAGIC_NUMBER {
            return Err(anyhow::anyhow!("Invalid Magic Number"));
        }
        Ok(header)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_header_serialization() {
        let original = FileHeader::new(
            "test_file.txt".to_string(),
            12345,
            4,
            "hash123".to_string(),
            10,
            2,
        );

        let bytes = original.to_bytes().expect("Serialization failed");
        assert_eq!(bytes.len(), HEADER_SIZE);

        let decoded = FileHeader::from_bytes(&bytes).expect("Deserialization failed");
        
        assert_eq!(decoded.magic, MAGIC_NUMBER);
        assert_eq!(decoded.version, VERSION);
        assert_eq!(decoded.original_filename, "test_file.txt");
        assert_eq!(decoded.file_size, 12345);
        assert_eq!(decoded.block_size, 4);
        assert_eq!(decoded.data_shards, 10);
        assert_eq!(decoded.parity_shards, 2);
    }
}

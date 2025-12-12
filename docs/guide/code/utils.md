# Utils Module (`src/utils.rs`)

This module defines the core data structures and constants shared between the Encoder and Decoder.

## Constants
- **MAGIC_NUMBER**: `0x4554_524E_4C53_5452` ("ETRNLSTR") identifies the file format.
- **HEADER_SIZE**: 1024 bytes. We reserve a fixed size for the header to make reading it predictable.
- **VIDEO_WIDTH / HEIGHT**: 1920x1080. We stick to 1080p to maximize compatibility with video players and hosting sites.

## The `FileHeader` Struct
Every video file starts with this metadata structure, serialized as JSON.

```rust
pub struct FileHeader {
    pub magic: u64,           // Verification
    pub version: u32,         // Format version (v1)
    pub original_filename: String,
    pub file_size: u64,       // Bytes
    pub block_size: u32,      // Pixel block size used
    pub sha256_hash: String,  // For integrity check
    pub data_shards: usize,   // RS config
    pub parity_shards: usize, // RS config
}
```

### Why JSON?
We use JSON (via `serde`) for flexibility. If we want to add fields later (e.g., encryption salt), we can do so without breaking basic parsing.

# Eternal-Stream (b2v)

![License](https://img.shields.io/badge/license-MIT-blue.svg)
![Rust](https://img.shields.io/badge/built_with-Rust-orange.svg)
![Status](https://img.shields.io/badge/status-stable-green.svg)

**Eternal-Stream (b2v)** is a CLI tool that encodes arbitrary binary files into video files and decodes them back. It is a spiritual successor to [Infinite Storage Glitch](https://github.com/DvorakDwarf/Infinite-Storage-Glitch), rewritten from scratch in Rust.

The tool reads input data in chunks (avoiding loading the whole file into RAM), applies Reed-Solomon forward error correction, and pipes raw frames to FFmpeg to produce a standard video container.

## Features

- **Chunk-based streaming**: Reads input files in chunks rather than loading them entirely into memory.
- **Reed-Solomon error correction**: Splits each chunk into data and parity shards so that some frame corruption can be recovered during decoding.
- **Configurable block size**: Each logical bit is expanded into a block of pixels (e.g., 4×4). Larger blocks trade storage density for resilience against lossy video compression.
- **Parallel frame encoding**: Uses Rayon to render pixel blocks across CPU cores.
- **FFmpeg backend**: Supports any codec FFmpeg accepts (tested with `ffv1` for lossless output and `libx264` for compressed output).

## Documentation

For detailed information about the architecture, code structure, and implementation details, please refer to the following documentation sections:

- [Architecture Overview](docs/guide/architecture.md)
- [Encoder Implementation](docs/guide/code/encoder.md)
- [Decoder Implementation](docs/guide/code/decoder.md)
- [Utility Functions](docs/guide/code/utils.md)

## Known limitations

- Output resolution is fixed at 1920×1080.
- The SHA-256 hash stored in the video header is a placeholder (`PENDING`). The actual hash is printed to stdout after encoding and is not verified automatically during decoding.
- The header frame is always encoded with block size 4, regardless of the `--block-size` value chosen for data frames.
- Lossy codecs (e.g., `libx264`) can corrupt data even with error correction if the compression is aggressive. Use `ffv1` for reliable round-trips.

## Installation

### Prerequisites
- Rust (1.70+)
- FFmpeg (must be in your PATH)

### Build from source
```bash
git clone https://github.com/fabriziosalmi/b2v.git
cd b2v
cargo build --release
```

The binary will be available at `./target/release/b2v`.

## Usage

### Example Usage

#### Encode (file to video)
Convert a binary file into a video file (`.mkv`, `.mp4`).

```bash
b2v encode \n  --input ./backup.iso \n  --output ./backup_video.mkv \n  --block-size 4 \n  --codec ffv1
```

#### Decode (video to file)
Restore the original file from a video.

```bash
b2v decode \n  --input ./backup_video.mkv \n  --output ./restored_backup.iso
```

### Options

| Option | Default | Description |
|--------|---------|-------------|
| `--input`, `-i` | Required | Path to the input file. |
| `--output`, `-o` | Required | Path to the output video. |
| `--block-size` | `4` | Pixel block size per bit. `1` gives highest density; larger values add resilience against compression. |
| `--codec` | `ffv1` | FFmpeg codec. `ffv1` is lossless; `libx264` is lossy. |
| `--data-shards` | `10` | Number of Reed-Solomon data shards per chunk. |
| `--parity-shards`
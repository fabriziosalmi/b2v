# Getting Started

## Introduction
Eternal-Stream (b2v) encodes arbitrary binary files into video files (`.mkv`, `.mp4`) and decodes them back. The video frames contain the binary data rendered as black-and-white pixel blocks, so the result looks like visual noise to a human viewer.

::: warning Compliance
Do not use this tool on public video hosting platforms without verifying their Terms of Service. Storing arbitrary binary data often violates fair-use and content policies. This tool is intended for [self-hosted storage](/guide/recommended-platforms) or private servers where you control the data.
:::

## Installation
### Via Cargo
If you have Rust installed:
```bash
cargo install --path .
```

## Basic Usage
### Encoding
To encode a file into a video:

```bash
b2v encode -i data.zip -o backup.mkv
```

This creates `backup.mkv`. Data is read in chunks, so memory usage stays low regardless of file size.

### Decoding
To restore a file from a video:

```bash
b2v decode -i backup.mkv -o restored_data.zip
```

The decoder reads the header from the first video frame to determine the encoding parameters automatically.

## Example Usage
### Encoding a Large File
To encode a 10GB file:
```bash
b2v encode -i large_file.bin -o output.mkv
```
This will process the file in 1MB chunks, ensuring low memory usage.

### Decoding with Verification
To decode and verify the integrity of a file:
```bash
b2v decode -i backup.mkv -o restored_data.zip
```
After decoding, compare the SHA-256 hash of the original file with the one printed during encoding.

## Notes
- The SHA-256 hash printed after encoding is not stored in the video header (the header contains a `PENDING` placeholder). Keep it separately if you want to verify integrity after decoding.
- Use `ffv1` (lossless) for reliable round-trips. Lossy codecs like `libx264` can corrupt data even with error correction if the compression is aggressive.
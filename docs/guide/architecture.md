# Architecture

## Pipeline Overview

### 1. The Encoder
1. **Input Stream**: Reads the input file in 1MB chunks.
2. **Reed-Solomon FEC**: Splits data into `10` data shards and calculates `2` parity shards (configurable). This allows recovering the file even if 20% of the data in a frame is lost.
3. **Block Scaling**: Each logical bit is expanded into a block of pixels (e.g., 4x4). This makes the signal robust against video compression algorithms (H.264/VP9) which blur high-frequency noise.
4. **FFmpeg Pipe**: The raw pixel frames are piped into `ffmpeg` via stdin to generate the video container (`mkv` or `mp4`).

### 2. The Decoder
1. **FFmpeg Pipe**: Spawns `ffmpeg` to read the video file and output raw RGB frames.
2. **Bit Extraction**: Scans the center of each pixel block to determine if it is a `0` or `1`, effectively downsampling the image.
3. **Header Parsing**: The first frame(s) contain a JSON header with file metadata (Filename, Size, Hash).
4. **Reconstruction**: Uses the Reed-Solomon engine to rebuild missing or corrupted shards.
5. **Output**: Writes the reconstructed bytes to the output file.

## Zero-Copy Design
The tool is designed to handle files larger than available RAM. It uses streaming iterators and buffers only a few frames at a time.

# Getting Started

## Introduction
Eternal-Stream (b2v) encodes arbitrary binary files into video files (`.mkv`, `.mp4`) and decodes them back. The video frames contain the binary data rendered as black-and-white pixel blocks, so the result looks like visual noise to a human viewer.

::: warning Compliance
Do not use this tool on public video hosting platforms without verifying their Terms of Service. Storing arbitrary binary data often violates fair-use and content policies. This tool is intended for [self-hosted storage](/guide/recommended-platforms) or private servers where you control the data.
::: 

*For detailed architectural insights, please refer to the [Architecture guide](/guide/architecture.md).* 

## Installation

### Via Cargo
If you have Rust installed:
```bash
cargo install --path .
```

### From source
1. Clone the repository
2. Run `cargo build --release`
3. The binary is in `target/release/b2v`
# Getting Started

## Introduction
Eternal-Stream (b2v) allows you to use video hosting platforms as infinite cloud storage. It works by converting binary files into video streams that look like static (random noise) to the human eye but contain error-corrected data for the computer.

::: warning ⚠️ COMPLIANCE WARNING
**Do not use this tool on public platforms without verifying their Terms of Service.**
Storing binary data as video often violates "Fair Use" policies of free hosting providers. We recommend using this tool for **[self-hosted storage](/guide/recommended-platforms)** or private servers only. You are responsible for your own data and accounts.
:::

## Installation

### Via Cargo
If you have Rust installed:
```bash
cargo install --path .
```

### From Source
1. Clone the repository
2. Run `cargo build --release`
3. The binary is in `target/release/b2v`

## Basic Usage

### Encoding
To backup a file:

```bash
b2v encode -i data.zip -o backup.mkv
```

This will create `backup.mkv`. The process runs in parallel and streams data, so it requires very little RAM.

### Decoding
To restore a file:

```bash
b2v decode -i backup.mkv -o restored_data.zip
```

The tool reads the metadata header from the video file automatically.

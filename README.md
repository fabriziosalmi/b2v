# Eternal-Stream (b2v)

![License](https://img.shields.io/badge/license-MIT-blue.svg)
![Rust](https://img.shields.io/badge/built_with-Rust-orange.svg)
![Status](https://img.shields.io/badge/status-stable-green.svg)

**Eternal-Stream (b2v)** is a high-performance Enterprise CLI tool designed to store large files as video streams on any video hosting platform.

It is a spiritual successor to [Infinite Storage Glitch](https://github.com/DvorakDwarf/Infinite-Storage-Glitch), rewritten from scratch in **Rust** with a focus on:
- **Performance**: Zero-Copy streaming and parallel processing (rayon).
- **Integrity**: Built-in Reed-Solomon Forward Error Correction (FEC).
- **Scalability**: Designed for 100GB+ backups using `ffv1` (lossless) or `libx264`.

## 🚀 Features

- **Streaming Architecture**: Never loads the whole file into RAM. Process Terabytes of data with minimal memory footprint.
- **Robust Error Correction**: Uses Reed-Solomon erasure coding to recover data even if video frames are corrupted or dropped.
- **Block Scaling**: Configurable pixel block size (e.g., 4x4) to resist video compression artifacts.
- **Cross-Platform**: Windows, Linux, MacOS (requires FFmpeg).

## 📦 Installation

### Prerequisites
- **Rust** (1.70+)
- **FFmpeg** (must be in your PATH)

### Build from Source
```bash
git clone https://github.com/fabriziosalmi/b2v.git
cd b2v
cargo build --release
```

The binary will be available at `./target/release/b2v`.

## 🛠 Usage

### Encode (File -> Video)
Convert a binary file into a video file (`.mkv`, `.mp4`).

```bash
b2v encode \
  --input ./backup.iso \
  --output ./backup_video.mkv \
  --block-size 4 \
  --codec ffv1
```

| Option | Default | Description |
|--------|---------|-------------|
| `--input`, `-i` | Required | Path to the input file. |
| `--output`, `-o` | Required | Path to the output video. |
| `--block-size` | `4` | Size of pixel blocks. `1` is densest, `8` is most robust. |
| `--codec` | `ffv1` | FFmpeg codec. `ffv1` (lossless) or `libx264` (compressed). |
| `--data-shards` | `10` | RS Data chunks per frame. |
| `--parity-shards` | `2` | RS Parity chunks for recovery. |

### Decode (Video -> File)
Restore the original file from a video.

```bash
b2v decode \
  --input ./backup_video.mkv \
  --output ./restored_backup.iso
```

*Note: The tool automatically reads the header from the video to determine original filename, size, and settings.*

## 🧪 Testing

Run the end-to-end verification script to confirm everything is working:

```bash
./test_e2e.sh
```

## ⚠️ Legal Disclaimer & Terms of Service

**CRITICAL WARNING**: Using this tool to store non-video data on **public video hosting platforms** effectively treats them as "Infinite Cloud Storage", which may likely **violate their Terms of Service (ToS)**.

We strongly recommend using `b2v` ONLY on:
- **Private/Self-hosted video instances** (e.g., your own Peertube instance, Nextcloud, NAS).
- **Platforms where you explicitly own the storage rights**.

**The authors of Eternal-Stream are not responsible for any banned accounts, data deletion, or legal consequences resulting from the misuse of this tool on third-party platforms.** Always evaluate the risks and read the ToS of the service you are using.

## 🤝 Recommended Platforms
To ensure data ownership and compliance, we recommend using this tool with Open Source, self-hosted alternatives:

| Platform | Type | Use Case |
|----------|------|----------|
| **[PeerTube](https://joinpeertube.org/)** | Decentralized Video | Community/Enterprise Hosting |
| **[Nextcloud](https://nextcloud.com/)** | Cloud Storage | Private Cloud Integration |
| **[Jellyfin](https://jellyfin.org/)** | Media Server | Home/NAS Backups |

## 📄 License
MIT License.

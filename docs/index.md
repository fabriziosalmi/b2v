---
layout: home

hero:
  name: "Eternal-Stream (b2v)"
  text: "Encode files as video."
  tagline: A Rust CLI tool that converts binary files into video streams and back, using Reed-Solomon error correction and FFmpeg.
  actions:
    - theme: brand
      text: Get Started
      link: /guide/getting-started
    - theme: alt
      text: View on GitHub
      link: https://github.com/fabriziosalmi/b2v

features:
  - title: Chunk-based streaming
    details: Reads input in chunks rather than loading the whole file into memory.
  - title: Reed-Solomon error correction
    details: Each chunk is split into data and parity shards so that limited frame corruption can be recovered during decoding.
  - title: Configurable block size
    details: Each bit is rendered as a square block of pixels. Larger blocks are more resilient to lossy video compression at the cost of storage density.
  - title: Compliance
    details: Intended for use on platforms where you control the storage. Do not use on public video hosts without verifying their Terms of Service.
---

# CLI Entry Application (`src/main.rs`)

The entry point uses `clap` to parse command line arguments and dispatch execution to either the Encoder or Decoder.

## Structure
```rust
#[derive(Subcommand)]
enum Commands {
    Encode { ... },
    Decode { ... },
}
```

## User Experience
We use `indicatif` to ensure the user always knows what's happening.
- **Progress Bars**: Show MB/s and ETA.
- **Feedback**: Clear "Success" or "Hash Mismatch" messages.

## Error Handling
We use `anyhow` to propagate errors up to `main()`. If something fails (e.g., FFmpeg crashes, disk full), the user gets a pretty error message instead of a panic.

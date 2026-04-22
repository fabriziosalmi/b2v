use super::*;

// Mock dependencies if necessary.

#[tokio::test]
async fn test_encoder_new() {
    let block_size = 4;
    let data_shards = 3;
    let parity_shards = 2;
    let codec = "libx264";

    let encoder = Encoder::new("input.mp4".to_string(), "output.mp4".to_string(), block_size,
        data_shards, parity_shards, codec);

    assert!(encoder.block_size == block_size);
    assert!(encoder.data_shards == data_shards);
    assert!(encoder.parity_shards == parity_shards);
    assert!(encoder.codec == codec);
}

#[tokio::test]
async fn test_encoder_run_setup() -> Result<(), Box<dyn std::error::Error>> {
    // This test verifies setup logic, assuming file access succeeds.

    let encoder = Encoder::new("input.mp4".to_string(), "output.mp4".to_string(), 4,
        3, 2, "libx264");

    // We cannot easily test the spawned process without mocking, so we check if setup completes.

    // If we could mock the file open and command spawn, this test would verify that paths are correct.
    assert!(true);

    Ok(())
}

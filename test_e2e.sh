#!/bin/bash
set -e

# ==============================================================================
# Eternal-Stream (b2v) E2E Verification Script
# ==============================================================================
# This script simulates the entire workflow of the project to ensure correctness.
# It acts as both a test suite and a demonstration of how the tool works.
#
# Workflow:
# 1. GENERATE: Creates a random binary file (simulate a backup).
# 2. ENCODE:   Converts the binary into a video file (MKV).
# 3. DECODE:   Reads the video and reconstructs the original binary.
# 4. VERIFY:   Compares the SHA256 hashes of input and output.
# ==============================================================================

# Configuration
INPUT_FILE="test_sample.bin"
VIDEO_FILE="test_output.mkv"
RESTORED_FILE="test_restored.bin"
SIZE_MB=10 # Size of the test file

echo "=========================================="
echo "    Eternal-Stream E2E Test Suite"
echo "=========================================="

# 0. Check pre-requisites
if ! command -v ffmpeg &> /dev/null; then
    echo "❌ Error: ffmpeg is not installed. Please install it (e.g. brew install ffmpeg)."
    exit 1
fi

if ! command -v cargo &> /dev/null; then
    echo "❌ Error: cargo is not installed. Please install Rust."
    exit 1
fi

# 1. Generate Sample File
echo ""
echo "[1/5] Generating random test data..."
echo "      Size: ${SIZE_MB}MB"
echo "      File: $INPUT_FILE"
dd if=/dev/urandom of=$INPUT_FILE bs=1M count=$SIZE_MB status=none
ORIGINAL_HASH=$(shasum -a 256 $INPUT_FILE | awk '{print $1}')
echo "      Hash: $ORIGINAL_HASH"

# 2. Build Project
echo ""
echo "[2/5] Building project (Release mode)..."
cargo build --release --quiet
if [ $? -ne 0 ]; then
    echo "❌ Build failed."
    exit 1
fi
echo "      Build successful."

# 3. Encode
echo ""
echo "[3/5] Encoding (Binary -> Video)..."
echo "      Command: b2v encode --input $INPUT_FILE --output $VIDEO_FILE --block-size 4 --codec ffv1"
./target/release/b2v encode --input $INPUT_FILE --output $VIDEO_FILE --block-size 4 --codec ffv1
if [ $? -ne 0 ]; then
    echo "❌ Encoding failed."
    exit 1
fi
echo "      Video created: $VIDEO_FILE"

# 4. Decode
echo ""
echo "[4/5] Decoding (Video -> Binary)..."
echo "      Command: b2v decode --input $VIDEO_FILE --output $RESTORED_FILE"
./target/release/b2v decode --input $VIDEO_FILE --output $RESTORED_FILE
if [ $? -ne 0 ]; then
    echo "❌ Decoding failed."
    exit 1
fi
echo "      File restored: $RESTORED_FILE"

# 5. Verify
echo ""
echo "[5/5] Verifying integrity..."
RESTORED_HASH=$(shasum -a 256 $RESTORED_FILE | awk '{print $1}')
echo "      Original Hash: $ORIGINAL_HASH"
echo "      Restored Hash: $RESTORED_HASH"

if [ "$ORIGINAL_HASH" == "$RESTORED_HASH" ]; then
    echo ""
    echo "✅ SUCCESS: Hashes match! Data integrity is 100% verified."
    echo "      Cleaning up temporary files..."
    rm $INPUT_FILE $VIDEO_FILE $RESTORED_FILE
    exit 0
else
    echo ""
    echo "❌ FAILURE: Hashes do not match! Data corruption occurred."
    echo "      Keep files for inspection."
    exit 1
fi

#!/bin/bash
set -e

# Configuration
INPUT_FILE="test_sample.bin"
VIDEO_FILE="test_output.mkv"
RESTORED_FILE="test_restored.bin"
SIZE_MB=10

echo "Expected Block Size: Default (4)"

# 1. Generate Sample File
echo "Generating ${SIZE_MB}MB random binary file..."
dd if=/dev/urandom of=$INPUT_FILE bs=1M count=$SIZE_MB status=none
ORIGINAL_HASH=$(shasum -a 256 $INPUT_FILE | awk '{print $1}')
echo "Original Hash: $ORIGINAL_HASH"

# 2. Build Project
echo "Building project..."
cargo build --release --quiet

# 3. Encode
echo "Encoding to $VIDEO_FILE..."
./target/release/b2v encode --input $INPUT_FILE --output $VIDEO_FILE --block-size 4
echo "Encoding completed."

# 4. Decode
echo "Decoding to $RESTORED_FILE..."
./target/release/b2v decode --input $VIDEO_FILE --output $RESTORED_FILE
echo "Decoding completed."

# 5. Verify
RESTORED_HASH=$(shasum -a 256 $RESTORED_FILE | awk '{print $1}')
echo "Restored Hash: $RESTORED_HASH"

if [ "$ORIGINAL_HASH" == "$RESTORED_HASH" ]; then
    echo "✅ SUCCESS: Hashes match! Data integrity verified."
    rm $INPUT_FILE $VIDEO_FILE $RESTORED_FILE
    exit 0
else
    echo "❌ FAILURE: Hashes do not match!"
    exit 1
fi

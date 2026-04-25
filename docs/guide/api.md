# API Documentation

## Rust APIs

### Decoder Module (src/decoder.rs)
- `decode_message(data: &[u8]) -> Result<Vec<u8>, DecodeError>`
  - Decodes binary data using the specified protocol
  - Returns decoded message or error if invalid format

### Encoder Module (src/encoder.rs)
- `encode_message(data: &[u8]) -> Vec<u8>`
  - Encodes binary data according to the protocol specification
  - Returns encoded message as Vec<u8>

### Utils Module (src/utils.rs)
- `validate_checksum(data: &[u8]) -> bool`
  - Verifies the integrity of encoded data
  - Returns true if checksum matches, false otherwise

## TypeScript APIs

### Decoder Tests (src/decoder.test.ts)
- `testDecodeValidMessage()`
  - Validates decoding of a correctly formatted message
- `testDecodeInvalidFormat()`
  - Ensures proper error handling for invalid formats

### Encoder Tests (src/encoder.test.ts)
- `testEncodeValidMessage()`
  - Verifies encoding of valid data
- `testEncodeEmptyData()`
  + Ensures proper handling of empty input
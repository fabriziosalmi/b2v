# API Reference

## Public Functions

### `decode(data: string): Result<DecodedData, Error>`
- **Description**: Decodes input data using the specified decoding algorithm.
- **Parameters**:
  - `data`: The string to be decoded.
- **Returns**: A Result containing either the decoded data or an error.

### `encode(data: DecodedData): Result<string, Error>`
- **Description**: Encodes input data using the specified encoding algorithm.
- **Parameters**:
  - `data`: The decoded data to be encoded.
- **Returns**: A Result containing either the encoded string or an error.

## Modules

### `decoder`
- Contains functions for decoding data using various algorithms.

### `encoder`
- Contains functions for encoding data using various algorithms.
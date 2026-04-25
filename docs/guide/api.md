# API Documentation

## Public APIs

This document details the public APIs available in the b2v repository.

### Decoder API

- `decode(input: string): Result<DecodedData, Error>`
  - Converts input data into a decoded format
  - Returns either the decoded data or an error

### Encoder API

- `encode(input: DecodedData): Result<string, Error>`
  - Converts decoded data back into the original format
  - Returns either the encoded string or an error

### Utility API

- `validate(input: string): boolean`
  - Validates the input data format
  - Returns true if valid, false otherwise
# API Reference

## Public APIs

### Decoder Module
- `decode(input: string): Result<DecodedData, Error>`
  - Decodes input data using the specified decoding algorithm
  - Returns a Result type containing either decoded data or an error

### Encoder Module
- `encode(input: DecodedData): Result<string, Error>`
  - Encodes input data using the specified encoding algorithm
  - Returns a Result type containing either encoded string or an error

### Utility Functions
- `validateInput(input: string): boolean`
  - Validates input data format before decoding
- `formatOutput(output: string): string`
  - Formats output data for display or further processing
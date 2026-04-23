# API Reference

## Overview
This section provides detailed information about the B2V API, including function descriptions, parameters, return values, and usage examples.

## Functions

### `decode(input: string): Result<DecodedData, Error>`
- **Description**: Decodes the input string using the B2V decoding algorithm.
- **Parameters**:
  - `input`: The string to be decoded.
- **Returns**: A Result object containing either the decoded data or an error message.

### `encode(input: DecodedData): string`
- **Description**: Encodes the input data using the B2/C encoding algorithm.
- **Parameters**:
  - `input`: The data to be encoded.
- **Returns**: A string representing the encoded data.

### `validate(input: string): boolean`
- **Description**: Validates the input string to ensure it conforms to the B2V format.
- **Parameters**:
  - `input`: The string to be validated.
- **Returns**: A boolean indicating whether the input is valid.

## Modules

### `decoder`
- **Description**: Contains functions related to decoding data using the B2V algorithm.
- **Functions**:
  - `decode(input: string): Result<DecodedData, Error>`

### `encoder`
- **Description**: Contains functions related to encoding data using the B2/C algorithm.
- **Functions**:
  - `encode(input: DecodedData): string`

### `utils`
- **Description**: Contains utility functions for working with B2V data.
- **Functions**:
  - `validate(input: string): boolean`
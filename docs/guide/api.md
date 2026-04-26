# API Documentation

## Overview
This document provides detailed information about the public API of the b2v project. It covers all modules, functions, and their usage.

## Modules
### Decoder Module
- **Function**: `decode(input: string): Result<String, Error>`
  - Converts binary data to a human-readable format.

### Encoder Module
- **Function**: `encode(input: string): Result<String, Error>`
  - Converts human-readable data to binary format.

### Utils Module
- **Function**: `validate(input: string): boolean`
  - Validates input data format.

## Functions
### decode()
- **Description**: Converts binary data to a human-readable format.
- **Parameters**:
  - `input`: The binary data to decode.
- **Returns**: A Result containing the decoded string or an error.

### encode()
- **Description**: Converts human-readable data to binary format.
- **Parameters**:
  - `input`: The string to encode.
- **Returns**: A Result containing the encoded binary data or an error.

### validate()
- **Description**: Validates input data format.
- **Parameters**:
  - `input`: The string to validate.
- **Returns**: A boolean indicating whether the input is valid.
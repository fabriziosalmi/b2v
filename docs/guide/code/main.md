# Code Examples

## Decoder Usage

### Frontend (TypeScript)
```typescript
import { decode } from '@b2v/decoder';

const data = 'example data';
const decodedData = decode(data);
console.log(decodedData);
```

### Backend (Rust)
```rust
use b2v::decoder;

fn main() {
    let data = "example data";
    let decoded_data = decoder::decode(data);
    println!("Decoded data: {}", decoded_data);
}
```

## Encoder Usage

### Frontend (TypeScript)
```typescript
import { encode } from '@b2v/encoder';

const data = { key: 'value' };
const encodedData = encode(data);
console.log(encodedData);
```

### Backend (Rust)
```rust
use b2v::encoder;

fn main() {
    let data = serde_json::json!({ "key": "value" });
    let encoded_data = encoder::encode(data);
    println!("Encoded data: {}", encoded_data);
}
```

## Utility Functions
- [Utility Functions Documentation](docs/guide/code/utils.md)
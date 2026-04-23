# Utility Functions

## Common Utilities
- `formatData(data: any): string` - Formats data for display
- `validateInput(input: any): boolean` - Validates input data
- `parseJSON(json: string): object` - Parses JSON strings into objects

## Example Usage

### Frontend (TypeScript)
```typescript
import { formatData, validateInput } from '@b2v/utils';

const input = 'example input';
if (validateInput(input)) {
    const formatted = formatData(input);
    console.log(formatted);
}
```

### Backend (Rust)
```rust
use b2v::utils;

fn main() {
    let input = "example input";
    if utils::validate_input(input) {
        let formatted = utils::format_data(input);
        println!("Formatted data: {}", formatted);
    }
}
```

## Additional Resources
- [Main Code Examples](docs/guide/code/main.md)
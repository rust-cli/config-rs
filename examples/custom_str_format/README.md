# Custom String Format

Shows how to implement a custom format for parsing configuration from an in-memory string rather than a file.

## Usage

```sh
cargo run --example custom_str_format
```

## How it works

1. Implements the `Format` and `FileStoredFormat` traits for a custom `MyFormat`
2. The parser only recognizes the string `"good"` as valid configuration, producing a single key-value pair
3. Uses `File::from_str()` to load configuration from a string literal instead of a file path
4. Demonstrates error handling — a warning is printed for unrecognized input

## Key concepts

- **`File::from_str()`** — load configuration from a string rather than the filesystem
- **Custom parsing logic** — the `Format` trait can implement arbitrary parsing
- **Error handling** — gracefully handle malformed configuration data

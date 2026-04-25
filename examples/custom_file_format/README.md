# Custom File Format

Shows how to implement a custom file format for config-rs by parsing PEM certificate files.

## Usage

```sh
cargo run --example custom_file_format
```

## How it works

1. Implements the `Format` and `FileStoredFormat` traits for a custom `Pem` format
2. The parser scans PEM file contents for "PUBLIC" or "PRIVATE" keywords to determine the key type
3. Loads `files/public.pem` and `files/private.pem` as optional configuration sources
4. Deserializes into a `Settings` struct with `public_key` and `private_key` fields

## Key concepts

- **`Format` trait** — defines how to parse raw file contents into configuration values
- **`FileStoredFormat` trait** — associates file extensions with the format so files can be auto-detected
- **Optional sources** — using `.required(false)` so the app doesn't fail if a file is missing

## Files

- `files/public.pem` — RSA public key
- `files/private.pem` — RSA private key

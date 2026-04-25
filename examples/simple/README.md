# Simple

A minimal example showing how to load configuration from a file and environment variables.

## Usage

```sh
cargo run --example simple
```

Override values with environment variables:

```sh
APP_DEBUG=true APP_KEY="new-key" cargo run --example simple
```

## How it works

1. Loads `Settings.toml` as the base configuration
2. Overlays any environment variables prefixed with `APP_`
3. Deserializes the result into a `HashMap<String, String>`

## Configuration

**Settings.toml:**

```toml
debug = false
priority = 32
key = "189rjfadoisfj8923fjio"
```

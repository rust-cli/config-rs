# Async Source

Demonstrates loading configuration from an async source — in this case, an HTTP endpoint.

## Usage

```sh
cargo run --example async_source --features="json async"
```

## How it works

1. Starts a local HTTP server on `localhost:5001` that serves JSON configuration
2. Implements the `AsyncSource` trait with a custom `HttpSource` struct
3. The client waits 3 seconds for the server to start, then fetches configuration via HTTP
4. Parses the JSON response into a `Config` using a custom `Format` implementation

## Key concepts

- **`AsyncSource` trait** — defines how to asynchronously collect configuration values
- **Custom `Format`** — parses the HTTP response body into config values
- Uses `warp` for the server and `reqwest` for the client

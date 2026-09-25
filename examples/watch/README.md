# Watch

Demonstrates hot-reloading configuration when a file changes on disk.

## Usage

```sh
cargo run --example watch
```

Then edit `examples/watch/Settings.toml` in another terminal — the running process will detect the change and reload automatically.

## How it works

1. Loads initial configuration from `Settings.toml` into a static `RwLock<Config>`
2. Uses the `notify` crate to watch the file for modifications (polling every 2 seconds)
3. On file change events, re-reads the configuration and updates the shared state
4. The main loop continuously prints current settings to the terminal

## Configuration

**Settings.toml:**

```toml
debug = false
port = 3223
host = "0.0.0.0"
```

## Key concepts

- **`RwLock<Config>`** — thread-safe shared configuration that can be read concurrently
- **`notify` crate** — cross-platform filesystem event watching
- **Hot-reload pattern** — update configuration without restarting the application

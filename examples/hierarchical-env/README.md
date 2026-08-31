# Hierarchical Environment

A real-world pattern for layered configuration with environment-specific overrides.

## Usage

```sh
# Development mode (default)
cargo run --example hierarchical-env

# Production mode
RUN_MODE=production cargo run --example hierarchical-env

# Override individual values
APP_DEBUG=false cargo run --example hierarchical-env
```

## How it works

Configuration is loaded in layers, with each layer overriding the previous:

1. `config/default.toml` — base configuration shared across all environments
2. `config/{RUN_MODE}.toml` — environment-specific overrides (e.g., `development.toml`, `production.toml`)
3. `config/local.toml` — optional local overrides (not checked into version control)
4. Environment variables with `APP_` prefix — runtime overrides
5. Programmatic overrides — hardcoded values set in code

## Configuration structure

```rust
struct Settings {
    debug: bool,
    database: Database,      // url
    sparkpost: Sparkpost,    // key, token, url, version
    twitter: Twitter,        // consumer_token, consumer_secret
    braintree: Braintree,    // merchant_id, public_key, private_key
}
```

## Key concepts

- **Layered configuration** — build up settings from multiple sources with clear priority
- **Environment-specific files** — different defaults for development vs. production
- **Local overrides** — developer-specific settings that aren't committed to the repo

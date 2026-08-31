# Environment Variable Lists

Demonstrates parsing space-separated environment variables into a `Vec<String>`.

## Usage

```sh
APP_LIST="Hello World" cargo run --example env-list
```

## How it works

1. Sets up an `Environment` source with the `APP` prefix
2. Enables `try_parsing(true)` so values are parsed beyond plain strings
3. Configures `list_separator(" ")` to split values on spaces
4. Deserializes into an `AppConfig` struct with a `list: Vec<String>` field

## Key concepts

- **`list_separator()`** — defines the delimiter for splitting environment variable values into lists
- **`try_parsing()`** — enables automatic type parsing of environment variable values
- **`separator()`** — maps underscores in env var names to nested config keys

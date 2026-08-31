# Glob

Demonstrates three approaches to loading and merging multiple configuration files.

## Usage

```sh
cargo run --example glob
```

## How it works

Three strategies are shown for combining files from `conf/`:

1. **Individual calls** — add each file one at a time with `add_source()`
2. **Vector of files** — pass a `Vec<File>` as a single source
3. **Glob pattern** — use the `glob` crate to match `conf/*` and collect files dynamically

All three produce the same merged `HashMap<String, String>`. Files are merged in order, with later files overriding earlier ones.

## Configuration files

- `conf/00-default.toml` — base defaults
- `conf/05-some.yml` — YAML overrides
- `conf/99-extra.json` — final JSON overrides

The numeric prefixes control load order and therefore priority.

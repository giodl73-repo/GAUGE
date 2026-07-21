# Contributing

Keep GAUGE evidence-labelled, source-backed, and explicit about the difference
between analysis and engineering or advocacy.

```powershell
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace --locked
cargo run -p gauge-cli -- --help
```

Do not commit raw restricted datasets, credentials, local build state, or
uncited public claims.

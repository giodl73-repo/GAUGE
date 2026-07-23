# Contributing

Keep GAUGE evidence-labelled, source-backed, and explicit about the difference
between analysis and engineering or advocacy.

Useful public contributions include corridor source inventories, timetable or
frequency/span corrections, station-access evidence, host-railroad or operations
review, and safer public language that prevents timetable, construction,
endorsement, or advocacy drift. For local adaptations, start with
[`docs/adoption/README.md`](docs/adoption/README.md).

```powershell
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace --locked
cargo run -p gauge-cli -- --help
```

Do not commit raw restricted datasets, credentials, local build state, or
uncited public claims.

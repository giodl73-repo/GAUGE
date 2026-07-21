# Pulse 01: WP-001 `gauge-network` rail kernel

Status: pending. Executes WP-001 (see `docs/vtrace/WORK_PACKAGES.md`).

## Scope

The rail graph kernel — the pipeline primitive every other crate depends on.
Implements the load-bearing identity and connectivity invariants and the typed
dispatch basis (dedicated vs shared host) required by REQ-007.

## Planned changes

- `Cargo.toml` workspace (member `crates/gauge-network`).
- `crates/gauge-network/Cargo.toml` (deps: `petgraph`, `serde`, `thiserror`).
- `crates/gauge-network/src/lib.rs`: `Station`, `Segment` (with typed `Dispatch`
  enum), `Network`, `NetworkError`; `add_station`/`add_segment` (identity +
  validation); `station_count`, `segment_count`, `degree`, `is_connected`,
  `corridor_miles`, `trip_time_minutes`, connectivity/centrality proxy (DIM-04).

## Parent IDs

REQ-004/005/007 · SPEC-001/005 · IF-005 · PKG-001 · CR-001..008.

## Exit criteria

- Workspace compiles; `cargo test -p gauge-network` green.
- Tests cover: build network; degree; connectivity vs gap; corridor miles +
  trip-time; dispatch basis preserved (dedicated/shared); duplicate-station,
  non-positive miles/speed, unknown-station typed errors.
- No `unwrap`/`panic!` in lib paths except tests; `clippy -D warnings` clean.

## Validation

```powershell
cargo fmt --check
cargo clippy --workspace -- -D warnings
cargo test -p gauge-network
```

## VTRACE closeout (on completion)

VER-004/005/007 + EVID-CR-001..003 → passed; TRACE REQ-004/005/007 → implemented;
WORK_PACKAGES WP-001 → done; unblock WP-002.

## Status

Completed — the six-crate workspace and validation baseline are implemented.

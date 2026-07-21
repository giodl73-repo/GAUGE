# GAUGE Product Plan

## Thesis

Score existing US passenger-rail corridors against a transparent dimension
pool, identify measurable service gaps, and design Rail 2.0 interventions only
where the evidence supports them.

## Implemented product shape

- Six-crate Rust workspace covering network, corpus, score, tier, gap, and CLI.
- Evidence-labelled markdown corpus contracts.
- DIM-01..13 scoring and tier-SLA shortfall artifacts.
- Tail-versus-systemic gap classification.
- Deterministic tests and machine-readable CLI outputs.

## Current evidence

The first cited frequency analysis covers 12 US corridors and classifies the
majority-below-bar distribution as systemic. Broader corpus coverage, additional
dimensions, and proposal review remain future work.

## Next public work

1. Expand cited corridor coverage and publish reproducible input manifests.
2. Add additional service dimensions and sensitivity runs.
3. Promote the first gap-targeted concept through the full review panel.
4. Publish comparison reports without turning heuristic evidence into advocacy.

## Non-goals

- No engineering alignment, environmental review, procurement, or timetable.
- No claim that rail dominates driving or flying in every market.
- No uncited ridership, trip-time, cost, or modal-shift claim.
- No silent scope expansion to rescue the hypothesis.

## Validation

```powershell
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace --locked
cargo run -p gauge-cli -- --help
```

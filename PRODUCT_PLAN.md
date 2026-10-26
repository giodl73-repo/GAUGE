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

The corrected historical frequency illustration retains twelve model entries
and classifies their majority-below-bar distribution as a systemic model pattern.
All twelve physical frequency quantities are source-needed after the 2026-10-26
custody audit. Frequency does not measure actual span, spacing or operating days;
these entries do not establish national representativeness or an upgrade target.
Dated original timetable acquisition and independent source-to-score review
precede evidence promotion. Broader corpus coverage, additional dimensions, and proposal review remain future work.

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

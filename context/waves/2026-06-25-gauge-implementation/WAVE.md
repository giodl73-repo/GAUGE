# Wave: GAUGE Implementation

## Goal

Build the GAUGE pipeline from the accepted work packages (WP-001..006), one work
package per pulse, each compiling and testing green before the next starts.

## Thesis

The left side of the V is settled (`docs/vtrace/`). This wave is the implementation
build: turn accepted work packages into tested Rust crates, bottom-up, with every
pulse running the WP verification commands and recording evidence back into the
VTRACE trace.

## Pulse table

| Pulse | Work Package | Status | Outcome |
|------:|--------------|--------|---------|
| 01 | WP-001 `gauge-network` | done | Rail kernel: identity, connectivity, centrality, path/trip-time, typed dispatch basis. |
| 02 | WP-002 `gauge-corpus` | done | Corpus model + schema + sources + evidence labels. |
| 03 | WP-003 `gauge-score` | done | Dimension scoring DIM-01..13 + rubric record. |
| 04 | WP-004 `gauge-tier` | done | Tier T1–T4 + SLA conformance + tier-SLA gap. |
| 05 | WP-005 `gauge-gap` | done | Gap analysis + null result. |
| 06 | WP-006 `gauge-cli` | done | CLI orchestration + reproducible artifacts. |

## Success criteria

- Each work package meets its exit criteria and verification commands.
- Workspace stays green (`cargo fmt --check`, `cargo clippy -D warnings`,
  `cargo test --workspace`) after every pulse.
- `proof check .` stays clean.
- VTRACE trace/verification rows updated as each WP closes.

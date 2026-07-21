# Work Packages

## Scope

Repo: GAUGE. Six implementation work packages that build the pipeline bottom-up.
Each is sized for an implementing agent (implementation automation) to run end-to-end: pick the lowest
unblocked WP, satisfy entry criteria, implement only the named surfaces, run the
verification commands, meet exit criteria, record the pulse, commit.

Product boundary rule: VTRACE/work-package/proof/readiness/evidence concepts are
**not** product features. Do **not** add `work-package`, `prove`, `readiness`, or
`evidence` subcommands. The CLI's product surface is corpus/score/tier/gap only.

## Work Package Table

| ID | Objective | Parent IDs | Affected Surfaces | L0 / L1 / L2 | Status |
|---|---|---|---|---|---|
| WP-001 | Rail graph kernel: identity, connectivity, centrality, path/trip-time helpers | REQ-004/005/007, SPEC-001/005, IF-005, PKG-001, CR-001..008 | `Cargo.toml`, `crates/gauge-network/**` | L0: `cargo test -p gauge-network` / L1: workspace fmt+clippy+test / L2: n/a | done |
| WP-002 | Corpus model + `corpus/SCHEMA.md` + `data/sources.md` + evidence labels + hold/reject | REQ-001/002/003/005, SPEC-002/003/009, IF-001/002, PKG-002 | `crates/gauge-corpus/**`, `corpus/SCHEMA.md`, `data/sources.md` | L0: `cargo test -p gauge-corpus` + `proof check .` / L1: workspace / L2: n/a | done |
| WP-003 | Dimension scoring DIM-01..13 (0–10) + rubric version record | REQ-006, SPEC-004, IF-003, PKG-003 | `crates/gauge-score/**` | L0: `cargo test -p gauge-score` / L1: workspace / L2: n/a | done |
| WP-004 | Tier T1–T4 classification + SLA conformance (DIM-13) + tier-SLA gap | REQ-014/015, SPEC-011/012, IF-004, PKG-004 | `crates/gauge-tier/**` | L0: `cargo test -p gauge-tier` / L1: workspace / L2: n/a | done |
| WP-005 | Gap analysis: under-served-region finder + null result | REQ-008, SPEC-006, PKG-005 | `crates/gauge-gap/**` | L0: `cargo test -p gauge-gap` / L1: workspace / L2: n/a | done |
| WP-006 | `gauge` CLI: corpus/score/tier-sla/gap commands + reproducible artifacts | REQ-001, IF-006, PKG-006 | `crates/gauge-cli/**` | L0: `cargo run -p gauge-cli -- --help` / L1: workspace / L2: end-to-end demo | done |

## Work Package Details

### WP-001: Rail graph kernel

Objective: a `gauge-network` crate that models the network as a graph of stations
and segments and exposes the load-bearing metrics.

Parent requirements: REQ-004, REQ-005, REQ-007.
Parent specs: SPEC-001 (identity), SPEC-005/SPEC-RB-01 (reliability basis typed).
Boundary/interface: PKG-001, IF-005. Code rigor: CR-001..008.

Product files to create:

- `Cargo.toml` (workspace, members include `crates/gauge-network`).
- `crates/gauge-network/Cargo.toml` (deps: `petgraph`, `serde`, `thiserror`).
- `crates/gauge-network/src/lib.rs` with:
  - `Station { id, name, state }`, `Segment { id, miles, max_mph, dispatch }`
    where `dispatch` is a typed enum (`Dedicated` | `SharedHost`) — REQ-007.
  - `Network` over a `petgraph` graph + `id -> NodeIndex` index.
  - `add_station` (reject duplicate id), `add_segment` (reject unknown station /
    non-positive miles / non-positive max_mph), with a `NetworkError` enum.
  - `station_count`, `segment_count`, `degree(id)`, `is_connected(a, b)`,
    `corridor_miles(path)`, and a free-flow `trip_time_minutes(path)` helper
    (sum of segment miles / max_mph), plus a `connectivity`/centrality proxy for
    DIM-04.

Entry criteria:

- [ ] `cargo` toolchain available.
- [ ] No existing `crates/` (greenfield).

Exit criteria:

- [ ] Workspace compiles; `cargo test -p gauge-network` green.
- [ ] Tests cover: build network; degree; connectivity between linked stations and
      non-connectivity across a gap; corridor miles and trip-time helper; a
      `SharedHost` vs `Dedicated` segment is preserved (REQ-007); duplicate
      station, non-positive miles/speed, and unknown-station typed errors
      (CR-003/004).
- [ ] No `unwrap`/`panic!` in lib paths except tests (CR-006); `clippy -D warnings`
      clean (CR-005).

Verification commands:

```powershell
cargo fmt --check
cargo clippy -p gauge-network -- -D warnings
cargo test -p gauge-network
```

Validation levels:

| Level | Required | Commands / Evidence |
|---|---|---|
| L0 | yes | `cargo test -p gauge-network` |
| L1 | yes | `cargo fmt --check`, `cargo clippy --workspace -- -D warnings`, `cargo test --workspace` |
| L2 | no | n/a (no pipeline yet) |

Boundary control:

| Boundary ID | Allowed Changes | Forbidden Changes | Integration |
|---|---|---|---|
| PKG-001 | `crates/gauge-network/**`, workspace `Cargo.toml` | scoring, tier, CLI, corpus logic | no |

Git execution: branch `wp-001-network`; one commit `GAUGE: WP-001 rail kernel`;
push when L1 green; stop when exit criteria met (do not start WP-002).

VTRACE-only closeout: set EVID-004/005/007 to passed; mark TRACE rows REQ-004/005/007
`implemented`; record pulse.

Status: done (built via implementation automation changeset; tests green).

### WP-002: Corpus model, schema, sources, labels

Objective: `gauge-corpus` crate + `corpus/SCHEMA.md` + `data/sources.md` that
represent one corridor as a labelled, sourced, tiered corpus entry, and hold/reject
unidentified or uncited rows.

Parent: REQ-001/002/003/005, SPEC-002/003/009, IF-001/002, PKG-002.

Product surfaces: `crates/gauge-corpus/src/lib.rs` (`EvidenceLabel` enum;
`Quantity { value, unit, label, source_id }`; `CorpusEntry { id, type, termini,
states, tier, sla, quantities, scores }`; load/validate from markdown+frontmatter;
`validate()` → held/rejected reasons); `corpus/SCHEMA.md` (IF-001);
`data/sources.md` (IF-002) with one seed entry.

Exit criteria: `cargo test -p gauge-corpus` green (missing-id reject, uncited
quantity held, label preservation — CR-007); `proof check .` clean. Boundary PKG-002
(depends on PKG-001 types). Git: `wp-002-corpus`. Status: done.

### WP-003: Dimension scoring

Objective: `gauge-score` crate scoring DIM-01..13 on a 0–10 scale with a versioned
rubric; values provisional and labelled (no calibration yet).

Parent: REQ-006, SPEC-004, IF-003, PKG-003.

Product surfaces: `crates/gauge-score/src/lib.rs` (`Dimension` enum DIM-01..13,
`Score(f64)` bounded `[0,10]`, `Rubric { version, weights }`, scoring trait over a
`CorpusEntry`); default rubric v0 with recorded rationale (IF-003).

Exit criteria: `cargo test -p gauge-score` green; score-bounds invariant tested
(CR-004); rubric version present. Boundary PKG-003 (depends on PKG-001/002). Git:
`wp-003-score`. Status: done.

### WP-004: Tier classification + SLA conformance

Objective: `gauge-tier` crate classifying each corridor into T1–T4, attaching tier
SLA terms, computing DIM-13 conformance, and emitting tier-SLA gaps.

Parent: REQ-014/015, SPEC-011/012, DIM-13, IF-004, PKG-004.

Product surfaces: `crates/gauge-tier/src/lib.rs` (`Tier { T1..T4 }`, `Sla {
trip_time, frequency, reliability, access }` per tier, `classify(entry) -> Tier`,
`conformance(entry, network) -> Dim13` naming the dispatch basis (REQ-007),
`tier_sla_gap(entry) -> Option<Gap>`); `tiers.toml` SLA record (IF-004), values
labelled provisional.

Exit criteria: `cargo test -p gauge-tier` green (classification, a conforming
corridor, a shortfall producing a tier-SLA gap; SLA values labelled provisional).
Boundary PKG-004 (depends on PKG-001/003). Git: `wp-004-tier`. Status: blocked by
WP-003.

### WP-005: Gap analysis

Objective: `gauge-gap` crate that plots scored corridors in dimension space, finds
under-served regions, and records a car/air-dominant market as a labelled null
result (REQ-008). Integrates tier-SLA gaps from `gauge-tier`.

Parent: REQ-008, SPEC-006, PKG-005.

Product surfaces: `crates/gauge-gap/src/lib.rs` (`GapRegion`, `find_gaps(corpus,
rubric) -> Vec<GapRegion>`, `null_result` path; consume tier-SLA gaps from PKG-004).

Exit criteria: `cargo test -p gauge-gap` green (one found gap and one null-result
case, no forced rail case). Boundary PKG-005 (depends on PKG-003/004). Git:
`wp-005-gap`. Status: done.

### WP-006: CLI orchestration

Objective: `gauge` CLI exposing `corpus`, `score`, `tier-sla`, and `gap`
subcommands that drive the pipeline and emit reproducible artifacts with labels
preserved (REQ-001, IF-006).

Parent: REQ-001, IF-006, PKG-006.

Product surfaces: `crates/gauge-cli/src/main.rs` (clap subcommands; reads corpus,
runs score/tier/gap, writes artifacts; `--help` documents the product surface; no
VTRACE subcommands).

Exit criteria: `cargo run -p gauge-cli -- --help` lists product subcommands;
end-to-end run on a seed corpus regenerates artifacts deterministically (CR-008);
`cargo test --workspace` green. Boundary PKG-006 (depends on all). Git:
`wp-006-cli`. Status: done.

## Orphan Check

- [x] Every accepted `REQ-*` is assigned to a work package or dispositioned
      (REQ-009..013 → already_satisfied process; REQ-001..008/014/015 → WP-001..006).
- [x] Every accepted `SPEC-*` is assigned to a work package or verification item.
- [x] Every interface-changing work package names `IF-*` IDs.
- [x] Every package/module-changing work package names `PKG-*` boundary IDs.
- [x] Every critical-code work package names `CR-*` IDs (WP-001 explicitly; all
      inherit CR-001..008 via CODE_RIGOR).
- [x] Every work package has exit criteria and verification commands.
- [x] Every work package lists L0/L1/L2 requirements.
- [x] No work package is only "cleanup" without parent IDs.

## Role Review Notes

| Role Lens | Work-Package Impact | Disposition |
|---|---|---|
| Systems engineering / V&V | Each WP is self-contained, ordered, with concrete verification commands and exit criteria. | pass |
| Scope Keeper | Product/process split enforced; no VTRACE concept becomes a CLI subcommand. | pass |
| Software-assurance lens | WPs inherit CR-001..008; WP-001 pins the identity/connectivity invariants. | pass |
| Operations Officer / Freight-Host Realist | WP-001 makes the dispatch basis (dedicated/shared) typed data; WP-004 names it on DIM-13 conformance (REQ-007). | pass |

Fixed-point note: no actionable finding required a change. Work packages are
runnable and orphan-free. WP-001 is `ready`; the rest unblock in sequence.

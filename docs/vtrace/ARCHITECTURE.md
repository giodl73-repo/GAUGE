# Architecture

## Scope

Repo: GAUGE

Architecture type: target (greenfield). Describes the intended system shape that
satisfies `REQUIREMENTS.md` and `SPECIFICATION_BASELINE.md`. No components are
built yet; boundaries and dependency direction are decided here so work packages
can be allocated cleanly.

## Architecture Summary

GAUGE is a layered pipeline: public rail/travel data becomes a typed, labelled
corpus; the corpus becomes a rail network graph; the graph and corpus feed
dimension scoring; scores feed tier classification and SLA-conformance; tier/score
outputs feed gap analysis and conceptual design; everything passes the `.roles`
review gate before promotion. The rail graph kernel is the primitive every other
layer depends on, mirroring ROUTE and PYLON's network-first shape. Layers depend
strictly downward — no cycles — so each can be built and verified in isolation.

## Components

| Component | Boundary ID | Responsibility | Requirement IDs | Interfaces | Evidence |
|---|---|---|---|---|---|
| `gauge-network` (rail kernel) | PKG-001 | Typed graph (Station/Segment/Corridor), stable identity, connectivity metrics (DIM-04), trip-time/feasibility helpers. | REQ-004, REQ-005, REQ-007 | IF-005 (lib API) | future VER-004/005/007 |
| `gauge-corpus` (corpus + data) | PKG-002 | Corpus file IO + schema, `data/sources.md` registry, evidence labels. | REQ-001, REQ-002, REQ-003 | IF-001, IF-002 | future VER-001/002/003 |
| `gauge-score` (scoring) | PKG-003 | Dimension pool DIM-01..13, 0–10 scoring, rubric calibration + versioning. | REQ-006 | IF-003 | future VER-006 |
| `gauge-tier` (tier/SLA) | PKG-004 | Tier classification T1–T4, SLA terms, DIM-13 conformance, tier-SLA gaps. | REQ-014, REQ-015 | IF-004 | future VER-014/015 |
| `gauge-gap` (gap analysis) | PKG-005 | Plot dimension space, find under-served regions, record null results. | REQ-008 | (internal) | future VER-008 |
| `gauge-cli` (orchestration) | PKG-006 | Commands that drive the pipeline and emit artifacts. | REQ-001 (regen path) | IF-006 (CLI) | future VER-001 |
| review layer (`.roles/`) | — (docs, not a crate) | Parliament + editorial gate, scope boundary. | REQ-009, REQ-010, REQ-011 | review records | EVID-009/010/011 |

## Package / Language Boundaries

Detailed inventory belongs in `PACKAGE_BOUNDARIES.md` (deferred). Summary:

| Boundary ID | Crate / Module | Language | Responsibility | Allowed Dependencies |
|---|---|---|---|---|
| PKG-001 | `gauge-network` | Rust | Graph primitive + metrics | (none internal) |
| PKG-002 | `gauge-corpus` | Rust | Corpus IO, labels, sources | PKG-001 (types) |
| PKG-003 | `gauge-score` | Rust | Dimension scoring | PKG-001, PKG-002 |
| PKG-004 | `gauge-tier` | Rust | Tier + SLA conformance | PKG-001, PKG-003 |
| PKG-005 | `gauge-gap` | Rust | Gap/null analysis | PKG-003, PKG-004 |
| PKG-006 | `gauge-cli` | Rust | Orchestration | PKG-001..005 |

Dependency direction is strictly downward (CLI → gap → tier → score → corpus →
network). The review layer is docs/process, not a build dependency.

## Data Flow

```text
public sources (FRA / Amtrak / BTS-NTAD / Census)
  -> [FLETCH fetch + cache]            (planned external acquisition)
  -> gauge-corpus  (typed, labelled corpus entries; data/sources.md)
  -> gauge-network (rail graph; identity, connectivity, trip-time helpers)
  -> gauge-score   (DIM-01..13 scores, calibrated rubric)
  -> gauge-tier    (T1-T4 classification, SLA conformance / DIM-13)
  -> gauge-gap     (gap map, under-served regions, null results)
  -> design proposals  (conceptual Rail 2.0 upgrades)
  -> .roles review     (parliament + editorial gate)
  -> reports / artifacts
```

## Dependencies

| Dependency | Purpose | Boundary / Risk | Verification |
|---|---|---|---|
| `petgraph` | Graph data structure + algorithms in PKG-001. | Well-scoped; low risk. | future cargo test |
| `serde` / `csv` | Corpus + data IO in PKG-002. | Low risk. | future cargo test |
| `clap` | CLI in PKG-006. | Low risk. | future cargo test |
| FLETCH (portfolio) | Source-byte/paged data acquisition + cache manifests. | Planned; not yet wired. Avoid TRACKER-relative paths (CON). | intake + future gate |
| PROOF (portfolio) | Markdown QA for docs/artifacts. | Tool/CLI relationship, not runtime. | `proof check .` (active) |
| METIS-CORE / RLINE (portfolio) | Optional graph partitioning / shared kernels for gap analysis. | Deferred until gap wave. | deferred |

## Failure Modes

| Failure Mode | Impact | Mitigation | Evidence |
|---|---|---|---|
| Missing/blocked source for a corpus quantity. | Incomplete score. | Hold row with `source-needed` label (REQ-005); never fabricate. | future VER-005 |
| Element lacks stable identity. | Unsafe joins across layers. | Reject/hold at PKG-002 schema gate (SPEC-001). | future VER-004 |
| Rubric not yet calibrated. | Scores provisional. | Label provisional; calibrate from corpus (REQ-006). | future VER-006 |
| Reliability basis unknown (host vs dedicated). | OTP claim unfounded. | Require dispatch basis named; hold if unknown (REQ-007/SPEC-RB-01). | future VER-007 |
| Tier assigned on assumption. | False SLA conformance. | Hold corridor if tier inputs missing (OPS-006). | future VER-014 |

## Open Risks

- Operator/host data openness (SPEC-UNK-001/002) may force proxy-heavy early
  corpus, especially for OTP.
- FLETCH integration is planned, not proven; until then acquisition is manual.
- Partitioning needs (METIS-CORE) are unproven until the gap wave.

## Role Review Notes

| Role Lens | Architecture Impact | Disposition |
|---|---|---|
| Systems engineering / Scope Keeper | Layered, downward-only dependencies; review layer correctly modeled as docs, not a crate. | pass |
| Rail Civil Engineer | Trip-time/feasibility helpers live in the kernel; disconnected-station failure mode handled as a hold. | pass |
| Operations Officer / Freight-Host Realist | Reliability-basis-unknown failure mode forces a hold rather than a fabricated OTP. | pass |
| Optimization / network lens | Initial draft let `gauge-corpus` depend on `gauge-score`, risking a cycle; resolved by making score depend on corpus (one-way). | resolved |
| Citation Auditor / Numeracy Checker | No quantities or arithmetic asserted. | pass |

Fixed-point note: one actionable finding (potential dependency cycle) was raised
and applied by fixing the dependency direction. No unresolved critical/major
finding. Detailed package boundaries deferred to `PACKAGE_BOUNDARIES.md`.

# Interfaces

## Scope

Repo: GAUGE

Interface type: target (greenfield). Controls GAUGE's external and cross-layer
boundaries so future work packages cannot change them silently. IF-001..004
restate the `SPECIFICATION_BASELINE.md` public contracts; IF-005..006 add the crate
API and CLI introduced in `ARCHITECTURE.md`. None are implemented yet.

## Interface Inventory

| ID | Interface | Type | Owner | Consumers | Compatibility Rule | Verification |
|---|---|---|---|---|---|---|
| IF-001 | Corpus entry file | file (markdown + frontmatter) | PKG-002 | analysts, PKG-003/004/005, reviewers | Frontmatter keys additive; `id` immutable | future VER-004 / schema check |
| IF-002 | `data/sources.md` registry | file (registry) | PKG-002 / data steward | citation audit, all scored quantities | Source entries append/annotate; ids stable | future VER-003 / citation audit |
| IF-003 | Rubric version record | file | PKG-003 | scoring, calibration | Dimension set + weights versioned | future VER-006 / calibration record |
| IF-004 | Tier/SLA record | file | PKG-004 | tier classification, gap analysis | Tier set + SLA terms versioned; reassignment recorded | future VER-014 / schema check |
| IF-005 | `gauge-network` library API | API (Rust crate) | PKG-001 | PKG-002..006 | Public types/functions semver; breaking change is change-control | future VER-007 / cargo test |
| IF-006 | `gauge` CLI | CLI | PKG-006 | maintainers, agents, analysts | Subcommands/flags additive; output schemas versioned | future VER-001 / command review |

## Interface Details

### IF-001: Corpus entry file

Purpose: one passenger-rail corridor as a reviewable, scored, labelled record.

Inputs: frontmatter (`id`, `type`, `termini`, `states`, `tier`, `sla`, source rows)
+ a dimension-score block (DIM-01..13).

Outputs: a `validated`/`held`/`draft` corpus artifact joinable by `id`.

Errors: missing `id` or uncited quantity → held (REQ-005); type/scope drift →
Scope Keeper finding.

Versioning: frontmatter keys additive; `id` semantics immutable; schema lives in
`corpus/SCHEMA.md` (deferred).

Evidence: future VER-004; `proof check .` for doc integrity.

### IF-002: `data/sources.md` registry

Purpose: the single citation registry; every cited quantity resolves here.

Inputs: source entries (id, publisher, dataset, access, cadence).

Outputs: stable source ids referenced by corpus quantities.

Errors: cited quantity with no registry entry → Citation Auditor finding;
non-source value → must carry a label.

Versioning: append/annotate only; ids stable.

Evidence: future VER-003.

### IF-003: Rubric version record

Purpose: control the dimension pool and weights as calibration evolves.

Inputs: dimension set (DIM-01..13), weights, calibration rationale.

Outputs: a versioned rubric the corpus scores against.

Errors: retiring/adding a `DIM-*` or changing weights without a version bump →
change-control violation.

Versioning: explicit version + recorded rationale (REQ-006).

Evidence: future VER-006.

### IF-004: Tier/SLA record

Purpose: control the T1–T4 tier definitions and per-tier SLA terms.

Inputs: tier definitions, SLA terms (trip-time, frequency/span, reliability,
access), corridor tier assignments.

Outputs: a versioned tier/SLA contract used by classification and gap analysis.

Errors: changing a tier/SLA term or a corridor's tier without a record →
change-control violation.

Versioning: tier set + SLA terms versioned; reassignment recorded.

Evidence: future VER-014/015.

### IF-005: `gauge-network` library API

Purpose: the graph primitive every layer builds on.

Inputs: stations, segments/corridors with attributes and identity.

Outputs: graph queries — connectivity, centrality (DIM-04), path existence,
trip-time helpers.

Errors: typed errors for unknown/duplicate ids and bad input (no panics on
expected bad input).

Versioning: semver; breaking public API change is a change-control event.

Evidence: future VER-007 (cargo test).

### IF-006: `gauge` CLI

Purpose: orchestrate the pipeline and emit artifacts reproducibly.

Inputs: corpus/data paths, subcommands (e.g. corpus, score, tier-sla, gap).

Outputs: regenerated corpus/score/tier/gap artifacts with labels preserved.

Errors: non-zero exit + message on missing inputs or gate failure.

Versioning: subcommands/flags additive; output schemas versioned.

Evidence: future VER-001 (command review).

## Open Questions

| ID | Question | Disposition |
|---|---|---|
| IFQ-001 | Exact corpus frontmatter schema and `corpus/SCHEMA.md`. | Defer to first corpus wave. |
| IFQ-002 | CLI output formats (JSON/CSV/markdown) per subcommand. | Defer to IMPLEMENTATION_PLAN. |
| IFQ-003 | Whether FLETCH owns acquisition behind IF-006 or a separate adapter. | Defer to intake. |

## Role Review Notes

| Role Lens | Interface Impact | Disposition |
|---|---|---|
| Scope Keeper | IF-001..006 restate or extend controlled contracts; no scoring/design asserted. | pass |
| Configuration/change-control lens | Every interface has a compatibility rule and change-control trigger. | pass |
| Citation Auditor | IF-002 hardens the citation boundary; no quantities introduced. | pass |
| Rail Civil Engineer | IF-005 exposes connectivity/trip-time as typed queries with error handling, not panics. | pass |

Fixed-point note: no actionable finding required a change; interfaces are
consistent with SPEC public contracts and architecture boundaries. No unresolved
critical/major finding. Schema and output-format details deferred to IFQ-001..003.

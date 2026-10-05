# GAUGE Invariants

These entries summarize properties that must remain true for GAUGE corpus
entries, score artifacts, tier/SLA checks, gap runs, findings, review gates, and
future reuse.

## GAUGE-I-01: Identity And Source Tags Are Non-Optional For Promotion

**Status:** VERIFIED

**Claim:** Every promoted corpus entry has stable corridor identity and
source-labelled quantities.

**Why it matters:** Unidentified or uncited timetable, ridership, cost, or
reliability values can make corridor comparisons look safer than they are.

**Enforcement:** Corpus validation preserves source fields, rejects missing ids,
and holds unsafe uncited quantities.

**Evidence:** `corpus/SCHEMA.md`, `data/sources.md`,
`crates/gauge-corpus/src/lib.rs`, `docs/vtrace/VERIFICATION.md`, and
`cargo test --workspace --locked`.

## GAUGE-I-02: Empty Dimensions Stay Empty

**Status:** VERIFIED

**Claim:** Empty dimensions are emitted as `EmptyRegion`, not coerced to zero or
hidden as success.

**Why it matters:** Missing DIM-01..06 and DIM-08..13 evidence in the first
corpus could be mistaken for low scores or adequacy if the output collapsed
those dimensions.

**Enforcement:** The gap replay emits explicit empty regions for unassessed
dimensions.

**Evidence:** `cargo run -p gauge-cli -- gap --input corpus`,
`docs/findings/2026-06-frequency-span-of-service.md`,
`crates/gauge-gap/src/lib.rs`, and `docs/vtrace/VERIFICATION.md`.

## GAUGE-I-03: Systemic And Tail Regions Stay Separate

**Status:** VERIFIED

**Claim:** Majority deficits are classified as `SystemicRegion`, while genuine
minority tails remain `TailRegion`.

**Why it matters:** A 67% below-bar rail-frequency distribution requires a
different interpretation than a two-port or few-state tail.

**Enforcement:** `gauge-gap` computes share below threshold and tests
majority-deficit and concentrated-tail cases separately.

**Evidence:** `crates/gauge-gap/src/lib.rs`,
`docs/findings/2026-06-frequency-span-of-service.md`,
`docs/vtrace/VERIFICATION.md`, and `cargo test --workspace --locked`.

## GAUGE-I-04: Rail Dimensions Stay Separate

**Status:** VERIFIED

**Claim:** Frequency/span, trip time, reliability, access, host constraints,
benefit-cost, and tier/SLA dimensions remain separately scored.

**Why it matters:** A frequency finding should not imply trip-time,
reliability, host-railroad, engineering, environmental, or cost adequacy.

**Enforcement:** `gauge-score` owns DIM-01..13 as distinct dimensions, and the
first finding explicitly limits itself to DIM-07.

**Evidence:** `docs/vtrace/SPECIFICATION_BASELINE.md`,
`docs/findings/2026-06-frequency-span-of-service.md`,
`crates/gauge-score/src/lib.rs`, and `docs/vtrace/TRACE.md`.

## GAUGE-I-05: Public Reuse Requires Boundary Review

**Status:** MITIGATED

**Claim:** Public or downstream reuse requires scope boundary language and role
review before GAUGE output is treated as a decision artifact.

**Why it matters:** A useful rail diagnostic can be mistaken for an engineering
study, timetable, procurement plan, advocacy mandate, or endorsement.

**Enforcement:** README/product boundaries, VTRACE review, role panel, and
PITFALL tracking keep public reuse bounded. The retained PITFALL policy check
requires the finding to keep DIM-07-only scope, unassessed dimensions, no
authority language, and explicit role-review prerequisites visible.

**Evidence:** `README.md`, `PRODUCT_PLAN.md`, `.roles/ROLE.md`,
`docs/findings/2026-06-frequency-span-of-service.md`,
`docs/vtrace/REVIEW.md`, `docs/pitfall-boundaries.v1.json`,
`tests/check-pitfall-policy.ps1`, and `.pitfall/gauge-pitfalls.md`.

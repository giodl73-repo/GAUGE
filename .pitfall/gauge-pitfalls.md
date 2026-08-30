# GAUGE Pitfalls

These entries capture recurring passenger-rail evidence failure classes and map
them to GAUGE controls or open repo-local risks.

## GAUGE-PF-01: Track Presence Becomes Service Adequacy

**Status:** MITIGATED

**Pattern:** Existing trains, routes, stations, or tracks are treated as proof
that a corridor meets a Rail 2.0 service promise.

**Domain:** Corpus entries, DIM-01..13 scoring, service-tier claims, findings,
and public summaries.

**Detection difficulty:** Rail presence is visible and emotionally salient,
while frequency, reliability, span, access, host control, and tier/SLA evidence
require more structured review.

**Structural solution:** Keep service dimensions separate and require source
labels, comparison basis, and tier/SLA posture before adequacy claims.

**Evidence:** `README.md`, `CLAUDE.md`,
`docs/findings/2026-06-frequency-span-of-service.md`,
`docs/vtrace/SPECIFICATION_BASELINE.md`, and `crates/gauge-score/src/lib.rs`.

## GAUGE-PF-02: Majority Deficit Is Minimized As A Tail

**Status:** MITIGATED

**Pattern:** A bottom-quartile trigger fires on a corpus where most entries are
below the bar, and the result is narrated as a small tail instead of a systemic
deficit.

**Domain:** Gap analysis, DIM-07 frequency finding, CLI output, public narrative,
and cross-repo detector standardization.

**Detection difficulty:** Tail membership can be correct while the
interpretation is wrong when below-bar share is high.

**Structural solution:** Compute share below threshold and classify majority
deficits as `SystemicRegion`.

**Evidence:** `docs/findings/2026-06-frequency-span-of-service.md`,
`crates/gauge-gap/src/lib.rs`, `docs/vtrace/VERIFICATION.md`, and
`cargo test --workspace --locked`.

## GAUGE-PF-03: Empty Dimensions Become Zero Or Hidden Evidence

**Status:** MITIGATED

**Pattern:** Missing non-frequency dimensions are mistaken for zeros, adequacy,
or absence of issue.

**Domain:** CLI gap output, findings, VTRACE verification, role review, and
portfolio readiness scoring.

**Detection difficulty:** Empty evidence can disappear from summaries unless
the artifact carries explicit coverage gaps.

**Structural solution:** Emit explicit `EmptyRegion` gaps and keep the first
finding limited to DIM-07.

**Evidence:** `docs/findings/2026-06-frequency-span-of-service.md`,
`docs/vtrace/VERIFICATION.md`, `crates/gauge-gap/src/lib.rs`, and
`cargo run -p gauge-cli -- gap --input corpus`.

## GAUGE-PF-04: Historical Governance Text Lags Implementation

**Status:** MITIGATED

**Pattern:** Foundation VTRACE docs continue to describe GAUGE as greenfield,
pending, or code-free after the Rust workspace, corpus, CLI, and first finding
exist.

**Domain:** VTRACE trace/review/verification, architecture, interfaces, code
rigor, implementation wave, portfolio status scoring, and research packet.

**Detection difficulty:** Historical VTRACE docs are internally coherent, but
they can be mistaken for current status after implementation lands.

**Structural solution:** Add current-state implementation updates to VTRACE
trace/review/verification, architecture, interfaces, and code-rigor docs without
rewriting the historical foundation record.

**Evidence:** `docs/vtrace/VERIFICATION.md`, `docs/vtrace/TRACE.md`,
`docs/vtrace/REVIEW.md`, `docs/vtrace/ARCHITECTURE.md`,
`docs/vtrace/INTERFACES.md`, and `docs/vtrace/CODE_RIGOR.md`.

## GAUGE-PF-05: First Rail Finding Becomes Public Authority

**Status:** OPEN

**Pattern:** The cited frequency/span finding, CLI gap output, or portfolio
summary is treated as an engineering study, timetable, procurement plan,
advocacy mandate, FRA/Amtrak/state-DOT position, host-railroad position, or
funding instruction.

**Domain:** README, first finding, customer distribution, research paper,
partner reuse, and downstream portfolio summaries.

**Detection difficulty:** The finding is reproducible and useful, and it names
real corridors, which can invite authority claims beyond its labels.

**Structural solution:** Keep research-lab and no-authority language visible,
preserve DIM-07-only posture and unassessed dimensions, and require explicit
full-panel release review before treating the finding as a public decision
artifact.

**Evidence:** `README.md`, `PRODUCT_PLAN.md`,
`docs/findings/2026-06-frequency-span-of-service.md`,
`docs/vtrace/REVIEW.md`, and `.roles/ROLE.md`.

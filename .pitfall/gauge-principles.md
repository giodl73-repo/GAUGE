# GAUGE Principles

These entries summarize durable GAUGE decision rules for passenger-rail evidence,
service promises, gap analysis, review, and public-authority boundaries.

## GAUGE-P-01: Service Promise Beats Track Presence

**Status:** ACTIVE

**Statement:** Passenger rail adequacy depends on trip time, frequency, span,
reliability, access, host constraints, and connections; track or train presence
alone is not enough.

**Rationale:** A corridor can exist and still fail the service promise that makes
rail usable against driving or flying.

**Decision rule:** Promoted findings must name assessed dimensions, unassessed
dimensions, source labels, comparison basis, and service-tier posture.

**Evidence:** `README.md`, `CLAUDE.md`, `docs/vtrace/REQUIREMENTS.md`,
`docs/vtrace/SPECIFICATION_BASELINE.md`, and
`docs/findings/2026-06-frequency-span-of-service.md`.

## GAUGE-P-02: Gaps Are Found, Not Invented

**Status:** ACTIVE

**Statement:** GAUGE designs into measured corpus gaps and accepts rigorous null
results instead of expanding scope to rescue a preferred rail hypothesis.

**Rationale:** Rail advocacy can jump from enthusiasm to maps; GAUGE's value is
the evidence-gated middle.

**Decision rule:** A concept or design proposal must trace back to corpus scores,
gap artifacts, and review evidence.

**Evidence:** `CLAUDE.md`, `PRODUCT_PLAN.md`, `docs/vtrace/CONOPS.md`,
`docs/vtrace/TRACE.md`, and `crates/gauge-gap/src/lib.rs`.

## GAUGE-P-03: Evidence Labels Survive The Pipeline

**Status:** ACTIVE

**Statement:** Cited, proxy, source-needed, heuristic, held, and provisional
evidence states must survive corpus parsing, scoring, tier checks, gap analysis,
CLI output, and findings.

**Rationale:** Timetable, ridership, host-railroad, cost, and reliability data
can look authoritative even when one dimension is a proxy or early estimate.

**Decision rule:** Source ids, evidence labels, corridor ids, and dispatch basis
cannot be silently dropped or upgraded as data moves across crates.

**Evidence:** `data/sources.md`, `corpus/SCHEMA.md`,
`docs/vtrace/CODE_RIGOR.md`, `crates/gauge-corpus/src/lib.rs`, and
`crates/gauge-cli/src/main.rs`.

## GAUGE-P-04: Systemic Deficits Are Not Tails

**Status:** ACTIVE

**Statement:** A majority-below-bar distribution should be classified as
systemic, not minimized as a tail.

**Rationale:** The first DIM-07 finding names 8 of 12 corridors below the
frequency/span bar; that is a network-class deficit, not a minority tail.

**Decision rule:** Gap analysis must preserve empty regions, broad alarms, and
systemic-vs-tail classification separately.

**Evidence:** `docs/findings/2026-06-frequency-span-of-service.md`,
`docs/vtrace/VERIFICATION.md`, `crates/gauge-gap/src/lib.rs`, and
`crates/gauge-gap` tests.

## GAUGE-P-05: Research Output Is Not Rail Authority

**Status:** ACTIVE

**Statement:** GAUGE can support research, review, cited diagnostics, and
conceptual design, but it does not issue engineering studies, timetables,
procurement plans, advocacy briefs, or FRA/Amtrak/state-DOT/host-railroad
endorsements.

**Rationale:** A reproducible rail-service signal and a public decision artifact
have different owners, standards, and accountability requirements.

**Decision rule:** Public-facing outputs must retain research-lab framing,
source and corpus limits, and no-authority language before reuse or publication.

**Evidence:** `README.md`, `PRODUCT_PLAN.md`, `docs/vtrace/REVIEW.md`, and
`.roles/ROLE.md`.

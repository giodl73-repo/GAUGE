# Mission

## Scope

Repo: GAUGE

VTRACE adoption scope: establish the mission baseline for GAUGE before creating
requirements, specification baselines, trace rows, or work packages. This file is
the leftmost VTRACE artifact for the repo and anchors later `REQ-*`, `SPEC-*`,
`WP-*`, verification, and validation records. GAUGE is greenfield: this mission
defines intent ahead of any implementation, and implementation must trace back to
the needs and constraints below.

## Mission Need

| ID | Need | Success Criteria | Status |
|---|---|---|---|
| NEED-001 | GAUGE shall turn public rail and travel data (e.g. FRA, Amtrak, BTS/NTAD, Census) into a reproducible scored corpus of existing US passenger-rail corridors. | A maintainer can regenerate the active corpus, score, and gap artifacts from documented commands, with source/proxy/heuristic labels preserved. | accepted |
| NEED-002 | GAUGE shall identify and explain rail gaps — slow high-potential corridors, missing high-speed spines, frequency gaps, intermodal gaps, and reliability holes — without overstating the evidence. | Every material claim is tied to a data artifact, command, source label, confidence label, or review record. | accepted |
| NEED-003 | GAUGE shall convert analysis into defensible conceptual Rail 2.0 upgrade options, not engineering studies, environmental reviews, or advocacy briefs. | Proposed corridors and feature packages are labelled implemented, heuristic, simulated, planned, held, or deprecated, and their benefit-cost case is labelled before publication. | accepted |
| NEED-004 | GAUGE shall keep corridor and station identity stable as analysis moves from raw stations/segments to scored corridors, gap regions, and design proposals. | Element-bearing artifacts join through a stable station/segment/corridor identifier rather than a transient label, operator, or map id. | accepted |
| NEED-005 | GAUGE shall expose rail tradeoffs through adversarial review roles instead of hiding them behind a single score. | Parliament and editorial reviews can change claims, labels, next evidence steps, or promotion status. | accepted |
| NEED-006 | GAUGE shall report a rigorous null result as a valid finding. | When the scored corpus shows a market where car or air genuinely dominates, the artifacts say so rather than forcing a rail case. | accepted |
| NEED-007 | GAUGE shall classify the network into a four-tier service hierarchy (T1 HSR Spine, T2 Intercity Trunk, T3 Regional Connector, T4 Branch/Feeder) and define trip-time, frequency, and reliability SLAs per tier across the whole system, so that "is rail adequate in this market?" is answered against an explicit tier promise. | Every analyzed corridor carries a tier and a declared SLA, and adequacy claims are made against the tier SLA rather than an unstated baseline. | accepted |

## Users

| User | Need | Success Signal |
|---|---|---|
| GAUGE maintainer | Know which commands, artifacts, and review gates define the current truthful repo state. | A clean validation bundle runs and the resulting artifacts match the documented claims. |
| Rail/transport analyst | Inspect scored corridors, gaps, and evidence labels without reverse-engineering the implementation. | Scores, gap maps, and reports cite their source surfaces and confidence posture. |
| Rail planner / reviewer | Understand why a corridor, tier, or feature package is supported, held, or downgraded. | Each claim names the data, scenario, role review, and next evidence step that governs it. |
| Operations / reliability reviewer | See how GAUGE handles on-time performance, frequency, and shared-track dispatch conceptually. | Reliability and frequency claims expose their dispatch basis and evidence level, not just an aggregate score. |
| Host freight railroad | See whether shared-track capacity, dispatch priority, and access cost are represented honestly. | Shared-track assumptions are explicit and priced, not assumed free. |
| Equity / community reviewer | See who the network serves and bypasses, and station-siting access exposure. | Access and feeder-tier claims point to data or held evidence, not narrative alone. |
| Coding agent | Make scoped changes without drifting claims, artifacts, or review obligations. | Work packages name parent IDs, affected modules/data/docs, validation commands, and evidence rows before closure. |

## Operating Context

GAUGE will be a data corpus, review system, and research/design process for Rail
2.0, with an implementation built later from accepted VTRACE work packages. Work
happens inside a dirty portfolio checkout, so repo-local changes must stay scoped
and must not depend on TRACKER-relative paths for build correctness. GAUGE is not
yet a TRACKER submodule until intake completes.

This mission file does not yet assert any scored result. It creates the VTRACE
anchor that later requirements, specifications, and work packages trace back to.

The tiering frame (NEED-007) is deliberately ROUTE/PYLON-shaped: just as Interstate
2.0 treats highways and Grid 2.0 treats transmission as tiered service systems with
SLA promises, GAUGE treats passenger rail as T1–T4 service tiers with
trip-time/frequency/reliability SLAs spanning the whole network, so that a
high-speed spine and a rural feeder are each judged against the promise appropriate
to their tier.

## Constraints

| ID | Constraint | Rationale | Status |
|---|---|---|---|
| CON-001 | GAUGE public claims must stay bounded by implemented commands, generated artifacts, source labels, confidence labels, and review records. | Prevents planned, heuristic, or simulated work from reading as proof-grade evidence. | accepted |
| CON-002 | Element-bearing artifacts must preserve stable station/segment/corridor identity; operators, route names, and map ids are not primary keys. | Keeps scores, gaps, and proposals tied to stable physical identity. | accepted |
| CON-003 | Generated artifacts must name the source-of-truth data and commands that regenerate them. | Keeps the repo reproducible and prevents hand-edited generated outputs from becoming hidden state. | accepted |
| CON-004 | Source gaps, heuristic rows, simulated evidence, and human/owner review holds must remain visible status, not missing prose. | Keeps evidence debt actionable and traceable. | accepted |
| CON-005 | GAUGE implementation changes belong in this repo; TRACKER receives only intentional submodule pointer updates after intake. | Preserves portfolio snapshot discipline. | accepted |
| CON-006 | GAUGE must not claim construction readiness, ridership/operations validity of record, environmental clearance, or official agency/host-railroad endorsement. | Keeps the project framed as research, tooling, review, and conceptual design. | accepted |

## Non-Goals

- GAUGE is not an engineering study, alignment design, or environmental review.
- GAUGE is not an operating plan, timetable, or procurement plan.
- GAUGE is not an advocacy brief for a specific corridor, operator, or policy.
- GAUGE does not predict what FRA, Amtrak, states, or host railroads will build.
- GAUGE does not treat illustrative maps or heuristic forecasts as proof-grade
  evidence unless their evidence level says so.

## Success Criteria

| Criterion | Validation Method | Evidence Pointer |
|---|---|---|
| VTRACE mission needs are explicit enough to derive requirements. | Inspect this file before writing `REQUIREMENTS.md`. | future `EVID-*` |
| Mission needs cover corpus reproducibility, evidence posture, design boundaries, identity, review roles, null-result discipline, and tiered SLAs. | Cross-check against `README.md`, `PRODUCT_PLAN.md`, and `CLAUDE.md`. | future `EVID-*` |
| Later VTRACE artifacts can reference stable parent IDs. | `REQ-*` rows should cite `NEED-*` and `CON-*` IDs from this file. | future `TRACE.md` |

## Role Review Notes

| Role Lens | Mission Impact | Disposition |
|---|---|---|
| Scope Keeper | Mission stays at repo/system intent; it asserts no scores, gap findings, or design proposals. | pass |
| Citation Auditor | Mission makes no quantitative claims; source links are repo-local context artifacts. | pass |
| Numeracy Checker | Mission contains no arithmetic, trip-time, ridership, or cost claims. | pass |
| Rail Planner | Mission names national network connectivity, tiering, and public-interest intent. | pass |
| Operations/Reliability Officer | Mission requires reliability/frequency framing and dispatch-basis evidence (NEED-002, NEED-007). | pass |
| Freight-Host Realist | Initial draft underplayed shared-track ownership; resolved by adding the Host freight railroad user lens and CON-006 host-endorsement boundary. | resolved |
| Climate/Modal & Equity advocates | Mission names modal shift and access/equity as first-class via users and NEED-002/007. | pass |

Fixed-point note: one actionable finding (shared-track/host-railroad reality
under-represented) was raised and applied. No unresolved critical or major finding
remains. Deferred: dimension pool, scoring rubric, tier SLA thresholds, and
reliability methodology to REQUIREMENTS and SPECIFICATION_BASELINE.

## Source Links

- `README.md`
- `PRODUCT_PLAN.md`
- `CLAUDE.md`
- `.roles/ROLE.md`

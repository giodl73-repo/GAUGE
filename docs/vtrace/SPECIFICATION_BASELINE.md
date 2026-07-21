# Specification Baseline

## Scope

Repo: GAUGE

Baseline type: target (provisional)

Baseline date: 2026-06-25

VTRACE adoption scope: define the controlled behavior GAUGE intends to build — the
dimension pool, scoring scale, reliability basis, corpus schema, evidence labels,
tier model, and review gate — before architecture, interfaces, or implementation
planning. Because GAUGE is greenfield, every item is `target`, not observed
`current`. The dimension pool is **provisional**: dimensions and their basis are
controlled here, but per-dimension anchors and rubric weights calibrate from the
scored corpus (REQ-006) and are not asserted in this file. Future work packages
must cite `SPEC-*` / `DIM-*` IDs instead of making unanchored changes.

## Specification Sources

| Source | Evidence | Status | Notes |
|---|---|---|---|
| `README.md` | GAUGE thesis, hypothesis, pipeline. | target | Public-facing repo intent. |
| `PRODUCT_PLAN.md` | Scope, non-goals, method, waves. | target | Product framing. |
| `CLAUDE.md` | House rules, pipeline, quality bar, forbidden vocabulary. | target | Operating constraints. |
| `docs/vtrace/MISSION.md` | `NEED-*`, `CON-*`. | current | VTRACE mission source. |
| `docs/vtrace/CONOPS.md` | `OPS-*` scenarios. | current | VTRACE scenario source. |
| `docs/vtrace/REQUIREMENTS.md` | `REQ-001..015`, `DEF-001..004`. | current | VTRACE requirement source. |
| `.roles/ROLE.md` | Parliament + editorial review lenses. | current | Review-lane source. |

## Dimension Pool (`DIM-*`)

The candidate pool GAUGE scores existing passenger-rail corridors against. Each
dimension is scored 0–10. Anchors and weights are **calibrated from the corpus**
(REQ-006), not fixed here. "Primary basis" names where the input comes from;
"Default label" is the evidence posture a fresh value carries until upgraded with a
cited source (REQ-002, REQ-003).

| DIM ID | Dimension | What it measures | Primary basis | Default label |
|---|---|---|---|---|
| DIM-01 | Ridership Potential | Market size at the distance/density where rail wins. | Ridership data, Census | source-needed |
| DIM-02 | Trip-Time Competitiveness | Door-to-door rail time vs. car and short-haul air. | Timetables + distances + mode comparison | heuristic |
| DIM-03 | Modal-Shift / Decarbonization Leverage | Trips shifted from car/air and lifecycle carbon saved. | Mode-share + carbon studies | heuristic |
| DIM-04 | Network Connectivity | Graph centrality and megaregion linkage; feeds the trunk. | NTAD topology (computable) | implemented |
| DIM-05 | Population / Megaregion Density Served | Population within station catchments along the corridor. | Census (computable) | implemented |
| DIM-06 | Reliability / On-Time Performance | OTP and delay recovery, with dispatch basis named. | Amtrak OTP + host-track status | source-needed |
| DIM-07 | Frequency / Span of Service | Trains per day and span of service. | Timetables (computable) | implemented |
| DIM-08 | Alignment Feasibility / Capacity | Geometry, grade separation, and track capacity for the claimed speed. | NTAD geometry + planning studies | heuristic |
| DIM-09 | Electrification / State of Good Repair | Electrified vs. diesel and asset condition. | FRA / operator data | proxy |
| DIM-10 | Operating Efficiency | Equipment utilization, turn times, recovery margin. | Operator data | proxy |
| DIM-11 | Capital-Efficiency / Benefit-Cost | Benefit per unit capital. | Planning B/C studies | heuristic |
| DIM-12 | Equity / Access Exposure | Population served vs. bypassed; station-siting and EJ access. | Census, EPA EJScreen (computable) | implemented |
| DIM-13 | Tier-SLA Conformance | Degree the corridor meets its assigned tier's trip-time/frequency/reliability/access SLA (derived; shortfall = tier-SLA gap). | Tier model + DIM-01/02/06/07 | heuristic |

Calibration note (per REQ-006, OPS-002): after the first corpus pass, low-variance
or redundant dimensions are retired and informative ones promoted; the rubric
version records each change. The pool above is the v0 candidate set, not a final
rubric.

## Reliability Basis (resolves DEF-002 minimum)

| ID | Rule |
|---|---|
| SPEC-RB-01 | Reliability and frequency dimensions (DIM-06, DIM-07) name the operating basis — **dedicated track vs shared host (freight-owned) track** — on every derived claim (REQ-007). |
| SPEC-RB-02 | Shared-track delay may be modelled as a labelled proxy when host dispatch data is unavailable; the proxy status must be explicit. Default scope names the basis (DEF-002 remains open for explicit delay modelling). |

## System Tier Model (`T1–T4`) (resolves NEED-007 / REQ-014 / REQ-015)

GAUGE classifies the whole network into a four-tier service hierarchy — spanning
HSR spines to rural feeders — with a trip-time/frequency/reliability/access SLA per
tier. This is the Rail 2.0 analog of ROUTE's stop-first SLA network and PYLON's
transmission tiering. Speed bands are typical, not strict; role in the network
governs tier when speed is ambiguous.

| Tier | Name | Typical role / speed | SLA promise (target) |
|---|---|---|---|
| T1 | HSR Spine | Dedicated high-speed corridors (≥150 mph) between major metros. | Trip-time beats air city-pair; high all-day frequency; high OTP; electrified. |
| T2 | Intercity Trunk | Higher-speed intercity corridors (≈90–125 mph). | Trip-time competitive with driving; multiple daily frequencies; defined OTP; intermodal transfer. |
| T3 | Regional Connector | Regional/commuter service feeding the trunk. | Peak headway + span of service; reliable OTP; last-mile/transit connection. |
| T4 | Branch / Feeder | Rural and feeder lines, thruway-bus connections. | Minimum daily connectivity and access to the trunk network. |

Each tier's SLA is expressed over four contract terms, assessed by DIM-13:

| SLA term | Meaning | Backing dimensions |
|---|---|---|
| Trip-time competitiveness | How the tier's door-to-door time compares to car/air. | DIM-02 |
| Frequency / span | Trains per day and span the tier promises. | DIM-07 |
| Reliability | OTP the tier must hold, with dispatch basis named (dedicated vs shared). | DIM-06, DIM-10 |
| Access / connectivity | Network connection and station access the tier provides. | DIM-04, DIM-12 |

SLA values per tier are **target and provisional** — exact thresholds calibrate
with the rubric (REQ-006) and are not asserted here. A tier-SLA shortfall is a
first-class gap (REQ-015, OPS-006).

## Controlled Specification Items

| Spec ID | Parent REQ IDs | Type | C/T/D/U | Specification Statement | Verification Method | Validation Method | Owner | Risk | Status |
|---|---|---|---|---|---|---|---|---|---|
| SPEC-001 | REQ-004 / REQ-005 | architecture | target | Every element is keyed by a stable station/segment/corridor identifier; operator, route name, and map id are mutable presentation fields, not keys. | schema check / inspection | OPS-001 | GAUGE maintainer | high | accepted |
| SPEC-002 | REQ-001 / REQ-003 / REQ-014 | product | target | A corpus entry is one markdown file with frontmatter (id, type, termini, states, tier, sla, source rows) and a scored dimension block, regenerable from documented commands. | inspection / command review | OPS-001 | GAUGE maintainer | medium | accepted |
| SPEC-003 | REQ-002 | product | target | Every quantity carries an evidence label from {implemented, heuristic, simulated, proxy, planned, held, source-needed, confidence-limited}. | artifact inspection | OPS-001 / OPS-004 | GAUGE maintainer | medium | accepted |
| SPEC-004 | REQ-006 | product | target | The dimension pool is `DIM-01..DIM-13` scored 0–10; anchors and weights are calibrated from corpus variance and versioned, not fixed in this baseline. | calibration record / version diff | OPS-002 | GAUGE maintainer | high | accepted |
| SPEC-005 | REQ-007 | software | target | Reliability/frequency dimensions name the operating basis (dedicated vs shared host track) on each claim (SPEC-RB-01). | analysis / inspection | OPS-003 | operations reviewer | high | accepted |
| SPEC-006 | REQ-008 | product | target | A market where car/air dominates is recorded as a labelled null result; scope is not expanded to force a rail case. | gap-artifact inspection / review | OPS-003 | GAUGE maintainer | medium | accepted |
| SPEC-007 | REQ-009 / REQ-010 | ops | target | Promotable claims pass the 7-voice parliament and 3-role editorial gate, with ridership, trip-time, reliability, modal-shift, benefit-cost, equity, and host-track lenses represented. | review inspection | OPS-004 | review steward | medium | accepted |
| SPEC-008 | REQ-011 | product | target | Outputs carry a scope boundary: research/tooling/conceptual-design only; no construction readiness, environmental clearance, ridership validity of record, or endorsement. | editorial review | OPS-004 | GAUGE maintainer | medium | accepted |
| SPEC-009 | REQ-003 | data | target | `data/sources.md` is the citation registry; every cited quantity names a registry entry, and proxies/heuristics are labelled rather than cited. | citation audit | OPS-001 | data steward | high | accepted |
| SPEC-010 | REQ-012 / REQ-013 | ops | target | VTRACE deliverables advance one at a time to a `.roles` fixed point; GAUGE changes stay in the child repo until an intentional TRACKER pointer update after intake. | wave ledger / git status | OPS-005 | GAUGE maintainer | low | accepted |
| SPEC-011 | REQ-014 | product | target | Every analyzed corridor is classified into exactly one tier (T1–T4) per the System Tier Model and carries that tier's declared SLA terms. | schema check / inspection | OPS-006 | GAUGE maintainer | high | accepted |
| SPEC-012 | REQ-015 | software | target | Tier-SLA conformance (DIM-13) is assessed per corridor against its tier SLA; any shortfall is recorded as a tier-SLA gap and a market is not called adequate while an unaddressed shortfall stands. | analysis / gate / inspection | OPS-003 / OPS-006 | GAUGE maintainer | high | accepted |

## Public Contracts

| Contract ID | Spec IDs | Surface | Compatibility Rule | Change-Control Trigger | Verification Evidence |
|---|---|---|---|---|---|
| IF-001 | SPEC-001 / SPEC-002 | corpus file (markdown + frontmatter) | Frontmatter keys are additive; `id` is immutable. | Any key rename/removal or id-semantics change. | schema check (target) |
| IF-002 | SPEC-009 | `data/sources.md` (registry) | Source entries are append/annotate; ids stable. | Removing or re-pointing a source id. | citation audit (target) |
| IF-003 | SPEC-004 | rubric version record | Dimension set + weights versioned; changes recorded. | Retiring/adding a `DIM-*` or changing weights. | calibration record (target) |
| IF-004 | SPEC-011 / SPEC-012 | tier/SLA record | Tier set (T1–T4) and per-tier SLA terms are versioned; tier reassignment is recorded. | Changing a tier definition, SLA term, or a corridor's tier. | tier/SLA record (target) |

## Package / Language Allocation

| Spec IDs | Package / Module | Responsibility | Forbidden Responsibility | Validation Profile |
|---|---|---|---|---|
| SPEC-001 / SPEC-004 / SPEC-005 | rail kernel (future `gauge-network`) | Graph model, connectivity metrics (DIM-04), trip-time/feasibility helpers. | Scoring policy, evidence labels, review logic. | L1 |
| SPEC-002 / SPEC-003 / SPEC-009 | corpus + data layer | File schema, source registry, evidence labels. | Graph math, design proposals. | L0/L1 |
| SPEC-007 / SPEC-008 | review layer (`.roles`) | Parliament/editorial gate, scope boundary. | Computing scores. | L0 |
| SPEC-011 / SPEC-012 | tier/SLA layer | Tier classification, SLA terms, tier-SLA conformance (DIM-13). | Setting calibrated SLA thresholds without rubric. | L1 |

## Nonfunctional Constraints

| Constraint ID | Parent Spec IDs | Constraint | Threshold / Rule | Verification Method | Status |
|---|---|---|---|---|---|
| SPEC-NF-001 | SPEC-002 / SPEC-004 | Reproducibility | Active corpus/score/gap artifacts regenerate from documented commands with labels preserved. | command review | proposed |
| SPEC-NF-002 | SPEC-009 | No raw datasets committed | Raw/cache data is gitignored; only derived, cited artifacts are committed. | inspection | proposed |
| SPEC-NF-003 | SPEC-001 | Deterministic identity | Element ids are deterministic given source inputs. | inspection / test | proposed |

## Assumptions And Unknowns

| ID | Item | Impact | Disposition | Owner |
|---|---|---|---|---|
| SPEC-UNK-001 | Ridership (DIM-01) and OTP (DIM-06) depend on operator data of varying openness. | May force proxy/source-needed labels on early corpus rows. | discovery → `data/sources.md` | data steward |
| SPEC-UNK-002 | Host-railroad dispatch data (DIM-06) is rarely public. | Likely proxy for shared-track OTP at v0. | accept risk (labelled proxy) | operations reviewer |
| SPEC-UNK-003 | Benefit-cost (DIM-11) requires study assumptions. | Heuristic until grounded. | defer to corpus calibration | transport economist |
| SPEC-UNK-004 | Per-tier SLA thresholds (DIM-13). | Affects conformance scoring. | defer to calibration | GAUGE maintainer |

## Requirement-To-Spec Coverage

| Requirement ID | Spec IDs | Coverage Status | Notes |
|---|---|---|---|
| REQ-001 | SPEC-002, SPEC-NF-001 | covered | Regeneration path. |
| REQ-002 | SPEC-003 | covered | Evidence labels. |
| REQ-003 | SPEC-009 | covered | Citation registry. |
| REQ-004 | SPEC-001 | covered | Stable identity. |
| REQ-005 | SPEC-001 | covered | Hold/reject unidentified rows. |
| REQ-006 | SPEC-004, IF-003 | covered | Calibrated rubric. |
| REQ-007 | SPEC-005, SPEC-RB-01 | covered | Dispatch basis named. |
| REQ-008 | SPEC-006 | covered | Null result. |
| REQ-009 | SPEC-007 | covered | Review gate. |
| REQ-010 | SPEC-007 | covered | Stakeholder lenses. |
| REQ-011 | SPEC-008 | covered | Scope boundary. |
| REQ-012 | SPEC-010 | covered | Child-repo scoping. |
| REQ-013 | SPEC-010 | covered | One-at-a-time VTRACE. |
| REQ-014 | SPEC-011, IF-004 | covered | Tier classification + SLA. |
| REQ-015 | SPEC-012, DIM-13 | covered | Tier-SLA gap gating. |

## Spec-To-Verification Coverage

| Spec ID | Verification IDs / Commands | Expected Result | Evidence Pointer | Status |
|---|---|---|---|---|
| SPEC-001..012 | future `VER-*` in `VERIFICATION.md` | Each spec has a credible check (schema, command, inspection, or review). | future `EVID-*` | planned |

## Role Review Notes

| Role Lens | Spec Impact | Disposition |
|---|---|---|
| Scope Keeper | Baseline defines controlled behavior and a candidate pool; it asserts no scored corridor or design. | pass |
| Citation Auditor | No quantities asserted; "Primary basis" names where inputs come from; DIM default labels enforce citation discipline. | pass |
| Numeracy Checker | Only the 0–10 scale is named; no computed scores, trip times, ridership, or costs. | pass |
| Operations Officer | Reliability basis is controlled (SPEC-RB-01 / SPEC-005); explicit delay modelling deferred. | pass |
| Freight-Host Realist | Initial draft let DIM-06 read as if host-track OTP were measured; resolved by SPEC-RB-02 (shared-track proxy must be labelled) and SPEC-UNK-002. | resolved |
| Transport Economist | Benefit-cost (DIM-11) default label set to `heuristic`; SPEC-UNK-003 records the gap. | pass |
| Climate/Modal & Equity advocates | Modal shift (DIM-03) and equity exposure (DIM-12) are in the pool. | pass |

Fixed-point note: one actionable finding (shared-track OTP risked reading as
measured) was raised and applied via SPEC-RB-02 + SPEC-UNK-002. No unresolved
critical or major finding remains. Pool and SLA values are explicitly provisional;
calibration deferred to the corpus wave.

## Specification Gate

Decision: pass_with_risk

Required before implementation planning:

- [x] Every accepted `REQ-*` maps to one or more `SPEC-*` IDs or a recorded deferral.
- [x] Every implementation work package can name parent `SPEC-*` IDs or discovery status.
- [x] Public contracts have owners and change-control triggers.
- [~] Unknowns are resolved, blocked, deferred, or converted to discovery work (SPEC-UNK-001..004 are discovery/defer/accept-risk).
- [x] Verification and validation methods are credible for the controlled claim.

Rationale: the baseline is coherent enough to drive trace, verification, and the
review gate. Residual risk is concentrated in operator/host-data openness
(SPEC-UNK-001/002) and provisional dimension weights/SLA thresholds, all deferred
to the corpus calibration wave rather than blocking the minimum slice.

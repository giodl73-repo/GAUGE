# Requirements

## Scope

Repo: GAUGE

VTRACE adoption scope: derive initial repo-level requirements from
`docs/vtrace/MISSION.md` and `docs/vtrace/CONOPS.md`. These requirements describe
what GAUGE must satisfy as analysis and implementation proceed; they do not by
themselves authorize implementation work — that comes from accepted work packages.
Requirements stay at contract level and assert no scores or designs.

## Requirement Table

| ID | Requirement | Parent Need / Constraint / Scenario | Rationale | Priority | Owner | Verification Method | Status |
|---|---|---|---|---|---|---|---|
| REQ-001 | GAUGE shall maintain a documented regeneration path for the active corpus, score, and gap artifacts from public source data. | NEED-001 / CON-003 / OPS-001 | Reproducibility is the minimum condition for trusting generated claims. | must | GAUGE maintainer | inspection / command review | accepted |
| REQ-002 | GAUGE shall label every material quantity with an evidence posture (implemented, heuristic, simulated, proxy, planned, held, source-needed, confidence-limited). | NEED-002 / NEED-003 / CON-001 / CON-004 / OPS-001 / OPS-004 | Evidence labels prevent proxy or planned work from reading as proof. | must | GAUGE maintainer | artifact inspection / review | accepted |
| REQ-003 | GAUGE shall cite a declared source in `data/sources.md` for every quantity in a corpus entry, or mark it as a labelled proxy/heuristic. | NEED-001 / CON-003 / CON-004 / OPS-001 | Uncited numbers cannot be audited or regenerated. | must | data steward | citation audit / inspection | accepted |
| REQ-004 | GAUGE shall identify each element by a stable station/segment/corridor identifier, not by a transient label, operator, or map id. | NEED-004 / CON-002 / OPS-001 | Stable physical identity is required before scores, gaps, and proposals can be compared. | must | GAUGE maintainer | schema check / inspection | accepted |
| REQ-005 | GAUGE shall hold or reject any corpus or gap artifact that lacks a stable element identifier or a declared source label. | NEED-004 / CON-002 / CON-004 / OPS-001 | Mutable labels and uncited rows cannot safely join across analysis stages. | must | GAUGE maintainer | gate / data inspection | accepted |
| REQ-006 | GAUGE shall calibrate its scoring rubric from observed corpus variance and correlation, and record the rubric version and rationale for each change. | NEED-002 / NEED-005 / OPS-002 | Calibration must be evidence-driven and auditable, not asserted. | must | GAUGE maintainer | calibration record / version diff | accepted |
| REQ-007 | GAUGE shall ground reliability and frequency claims in an explicit operating basis (shared-track dispatch vs dedicated) and name the basis on the claim. | NEED-002 / CON-001 / OPS-003 / OPS-006 | On US rail, reliability is meaningless without stating whether the corridor is dedicated or runs on host freight track. | must | operations reviewer | inspection / review | accepted |
| REQ-008 | GAUGE shall record a market where car/air genuinely dominates as a valid null result rather than forcing a rail case. | NEED-006 / CON-001 / OPS-003 | Silent scope expansion to rescue a hypothesis is forbidden. | must | GAUGE maintainer | gap-artifact inspection / review | accepted |
| REQ-009 | GAUGE shall route every promotable corridor or feature-package claim through the 7-voice parliament and the 3-role editorial gate before downstream use. | NEED-005 / CON-001 / OPS-004 | GAUGE's review system is part of the evidence model, not decoration. | must | review steward | review inspection | accepted |
| REQ-010 | GAUGE shall represent ridership, trip-time competitiveness, reliability, modal-shift, benefit-cost, equity/access, and shared-track host posture in reviews or claim labels before a design option is promoted. | NEED-003 / NEED-005 / OPS-004 | These stakeholder lenses must remain first-class, per the mission users. | should | review steward | role review / inspection | accepted |
| REQ-011 | GAUGE shall keep its outputs framed as research, tooling, review, and conceptual design — not construction readiness, environmental clearance, ridership validity of record, or agency/host endorsement. | NEED-003 / CON-006 / OPS-004 | Scope control protects GAUGE from overclaiming public authority. | must | GAUGE maintainer | editorial review | accepted |
| REQ-012 | GAUGE shall keep implementation and VTRACE changes scoped to the GAUGE child repo until an intentional TRACKER submodule pointer update after intake. | CON-005 / OPS-005 | TRACKER is the snapshot repo; GAUGE owns implementation history. | must | GAUGE / portfolio maintainer | git status / submodule diff | accepted |
| REQ-013 | GAUGE shall advance VTRACE deliverables one at a time to a `.roles` review fixed point, recording dispositions and deferrals. | NEED-005 / OPS-005 | The one-at-a-time discipline keeps each artifact reviewable and traceable. | must | GAUGE maintainer | wave ledger / review notes | accepted |
| REQ-014 | GAUGE shall classify every analyzed corridor into exactly one tier (T1 HSR Spine, T2 Intercity Trunk, T3 Regional Connector, T4 Branch/Feeder) and attach the tier's declared SLA (trip-time, frequency/span, reliability, access). | NEED-007 / CON-002 / OPS-006 | A tiered SLA system requires every corridor to carry a tier and a promise it is judged against. | must | GAUGE maintainer | schema check / inspection | accepted |
| REQ-015 | GAUGE shall assess each corridor against its tier SLA and report any tier-SLA shortfall as a gap before a market is described as adequate. | NEED-007 / NEED-002 / NEED-006 / OPS-003 / OPS-006 | Adequacy must be measured against an explicit tier promise, not an unstated baseline; SLA gaps are first-class findings. | must | GAUGE maintainer | gate / gap-artifact inspection | accepted |

## Requirement Quality Checklist

- [x] Each requirement is clear.
- [x] Each requirement is feasible.
- [x] Each requirement is verifiable.
- [x] Each requirement has an owner.
- [x] Each requirement links to a mission need, constraint, or CONOPS scenario.
- [x] Each requirement avoids implementation detail unless the detail is itself required.

## Role Review Notes

| Role Lens | Requirement Impact | Disposition |
|---|---|---|
| Scope Keeper | Requirements stay at contract level; no corridor scored, gap proposed, or design specified. | pass |
| Citation Auditor | Requirements introduce no new numeric claims; REQ-003 hardens citation discipline. | pass |
| Numeracy Checker | No calculations, units, scores, trip-time, or ridership claims. | pass |
| Rail Planner | Connectivity and tiering intent preserved via REQ-014/REQ-010. | pass |
| Operations Officer | Initial draft left the reliability basis implicit; resolved by adding REQ-007 (explicit dispatch basis — dedicated vs shared host — named on the claim). | resolved |
| Freight-Host Realist | Host-track posture required before promotion (REQ-010); reliability basis named (REQ-007). | pass |
| Climate/Modal & Equity advocates | Modal-shift and equity/access required before promotion (REQ-010). | pass |

Fixed-point note: one actionable finding (reliability/dispatch basis implicit) was
raised and applied as REQ-007. No unresolved critical or major finding remains.

## CONOPS Trace Review

| Scenario ID | Requirements Derived |
|---|---|
| OPS-001 | REQ-001, REQ-002, REQ-003, REQ-004, REQ-005 |
| OPS-002 | REQ-006 |
| OPS-003 | REQ-007, REQ-008 |
| OPS-004 | REQ-002, REQ-009, REQ-010, REQ-011 |
| OPS-005 | REQ-012, REQ-013 |
| OPS-006 | REQ-014, REQ-015 |

## Deferred Requirements

| ID | Reason Deferred | Revisit Trigger |
|---|---|---|
| DEF-001 | Exact dimension pool and per-dimension definitions. | `SPECIFICATION_BASELINE.md` and first corpus-calibration wave. |
| DEF-002 | Whether reliability scoring models shared-track delay explicitly vs. as a proxy. | `SPECIFICATION_BASELINE.md` once the method is chosen. |
| DEF-003 | Specific data-source acquisition commands and refresh cadence. | `data/sources.md` and `VERIFICATION.md`. |
| DEF-004 | Implementation interfaces (CLI, schemas, crates). | `ARCHITECTURE.md` / `INTERFACES.md` after the minimum slice. |

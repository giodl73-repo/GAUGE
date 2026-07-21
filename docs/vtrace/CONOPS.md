# Concept of Operations

## Scope

Repo: GAUGE

VTRACE adoption scope: describe the operating scenarios that GAUGE's requirements
and specification baseline must preserve. CONOPS is the bridge from mission needs
(`MISSION.md`) to observable repo workflows. It asserts no scores, gaps, or
corridors. GAUGE is greenfield, so these scenarios describe intended operation,
and each names the mission need it serves.

## Actors

| Actor | Responsibility | Needs |
|---|---|---|
| GAUGE maintainer | Own repo truth, active goal, generated artifacts, and promotion posture. | Clear commands, evidence labels, review gates, scoped child-repo changes. |
| Coding agent | Make bounded changes to corpus, data, docs, and (later) code from accepted work packages. | Parent `NEED-*`/`REQ-*`/`WP-*` IDs, affected surfaces, validation commands, stop conditions. |
| Review steward | Run `.roles` review lanes; record changes to claims, holds, labels, or next steps. | Mission/requirement IDs, review scope, artifacts to inspect. |
| Rail/transport analyst | Inspect corpus scores, gap maps, and design options. | Reproducible artifacts with source, confidence, and evidence posture. |
| Data steward | Maintain `data/sources.md` and source/proxy/heuristic label discipline. | Stable source identifiers, update cadence, citation rules. |
| Stakeholder reviewer | Apply rail-planner, operations, economist, climate, equity, and freight-host lenses. | Claims that name ridership, trip-time, reliability, modal shift, access, and host-track posture. |

## Scenarios

### OPS-001: Add And Score An Existing Corridor (serves NEED-001, NEED-002, NEED-004)

Trigger: a maintainer or agent adds an existing passenger-rail corridor to the
corpus.

Inputs: public source data (FRA / Amtrak / BTS-NTAD / Census), the dimension pool,
the corpus schema, and `data/sources.md`.

Normal path:

1. Create one corpus file with a stable station/segment/corridor identifier.
2. Populate dimension values, each citing a source or labelled proxy/heuristic.
3. Score the corridor against the calibrated rubric.
4. Record source, confidence, and evidence labels on every quantity.

Failure or degraded path: if a source is missing or blocked, the row remains held
with a source-needed label and a next evidence step; it is not filled with uncited
prose.

Outputs: one scored corpus entry with preserved labels.

Handoffs: maintainer to review steward when evidence posture changes.

Validation evidence: PROOF/doc checks, source-label inspection, future `EVID-*`.

### OPS-002: Calibrate The Rubric (serves NEED-002, NEED-005)

Trigger: enough corridors are scored to test which dimensions differentiate.

Inputs: scored corpus, dimension pool, variance/correlation review.

Normal path:

1. Inspect per-dimension variance and cross-dimension correlation.
2. Retire low-variance or redundant axes; promote informative ones.
3. Bump the rubric version and record the rationale.

Failure or degraded path: if the corpus is too small or unbalanced, the pass is
deferred and the limitation recorded rather than forcing a change.

Outputs: a versioned rubric and a calibration record.

Handoffs: maintainer to analyst and review steward.

Validation evidence: calibration record, rubric version diff, future `EVID-*`.

### OPS-003: Build The Gap Map And Surface A Candidate (serves NEED-002, NEED-006)

Trigger: the calibrated corpus is plotted to find under-served regions.

Inputs: scored corpus, calibrated rubric, gap-analysis method.

Normal path:

1. Plot scored corridors in the dimension space.
2. Identify under-served regions (e.g. high ridership potential + slow trip time +
   low frequency).
3. Record candidate gaps with the dimensions and corpus comparison that define
   them.

Failure or degraded path: if a market shows car/air genuinely dominates, that null
result is recorded as a finding (NEED-006), not forced into a rail case.

Outputs: gap-map artifact and candidate-gap records, or a recorded null result.

Handoffs: maintainer to design author.

Validation evidence: gap artifact, reproduction command, future `TRACE.md`.

### OPS-004: Review And Promote Or Hold A Design Claim (serves NEED-003, NEED-005)

Trigger: a conceptual Rail 2.0 corridor/feature package is proposed for downstream
use.

Inputs: the proposal, evidence labels, parliament/editorial lenses, non-goal
constraints from `MISSION.md`.

Normal path:

1. Confirm the claim carries an evidence label (implemented / heuristic /
   simulated / planned / held / deprecated).
2. Run the 7-voice parliament; require each voice to challenge or accept —
   including the freight-host realist on any shared-track assumption.
3. Run the 3-role editorial gate (citation, scope, numeracy).
4. Promote only the bounded claim; keep construction readiness, environmental
   clearance, and endorsement out of scope.

Failure or degraded path: if evidence, review, or scope is insufficient, the claim
stays held or downgraded with a next evidence step.

Outputs: promoted, held, or downgraded claim plus a review record.

Handoffs: review steward to maintainer or design owner.

Validation evidence: review record, editorial-gate result, future `EVID-*`.

### OPS-005: Apply VTRACE One Deliverable At A Time (serves all NEEDs)

Trigger: a maintainer asks to advance GAUGE's VTRACE baseline.

Inputs: existing `docs/vtrace/` artifacts, `.roles`, VTRACE templates, the active
wave ledger.

Normal path:

1. Create or revise exactly one VTRACE deliverable.
2. Use prior VTRACE IDs as parent IDs.
3. Review against the relevant `.roles` subset to a fixed point.
4. Run doc validation; record the stage in the wave ledger.
5. Commit the child repo only when the stage is settled and validated.

Failure or degraded path: if repo state is dirty or out of scope, keep edits scoped
to the one deliverable and report status.

Outputs: one reviewed VTRACE artifact with stable IDs.

Handoffs: maintainer to next-stage author.

Validation evidence: `git diff --check`, PROOF check, role review notes.

### OPS-006: Classify Tier And Check SLA Conformance (serves NEED-007, NEED-002)

Trigger: a corridor is added or re-evaluated, or a market's adequacy is assessed.

Inputs: corridor attributes (speed, distance, service, ownership), the T1–T4 tier
model, and the per-tier SLA contract.

Normal path:

1. Classify the corridor into T1 (HSR Spine), T2 (Intercity Trunk), T3 (Regional
   Connector), or T4 (Branch/Feeder).
2. Look up the tier's SLA (trip-time, frequency/span, reliability, access).
3. Assess conformance: does the corridor meet its tier SLA, with the reliability
   basis named (dedicated vs shared host track, REQ-007)?
4. Record a tier-SLA gap where the corridor under-serves its tier promise.

Failure or degraded path: if tier or SLA inputs are missing, the corridor is held
with a source-needed label rather than assigned a tier on assumption.

Outputs: a tier label, an SLA-conformance assessment, and any tier-SLA gap.

Handoffs: maintainer to gap author and review steward.

Validation evidence: tier/SLA record, conformance check, future `EVID-*`.

## Operational Assumptions

- GAUGE is greenfield: VTRACE is authored ahead of implementation, so scenarios
  describe intended operation, not retrofit.
- The active VTRACE sequence is MISSION → CONOPS → REQUIREMENTS →
  SPECIFICATION_BASELINE before implementation planning.
- Some data sources may be expensive or blocked; GAUGE records intended
  acquisition and validation even when a full pass is deferred.
- `.roles` review is part of GAUGE operations and must change evidence posture,
  claim labels, or next steps when it finds a gap.
- TRACKER remains the portfolio snapshot repo; GAUGE owns repo-local implementation
  and VTRACE artifacts.

## Role Review Notes

| Role Lens | CONOPS Impact | Disposition |
|---|---|---|
| Scope Keeper | Scenarios describe workflows; no specific corridor/gap/design prescribed. | pass |
| Citation Auditor | No new quantitative claims; scenarios name repo-local artifacts and future evidence paths. | pass |
| Numeracy Checker | No arithmetic, units, score ranges, trip times, or ridership figures. | pass |
| Rail Planner / Operations Officer | Scoring and review scenarios require reliability and dispatch-basis evidence before promotion. | pass |
| Freight-Host Realist | OPS-004 explicitly requires the realist to challenge shared-track assumptions. | pass |
| Data steward (added lens) | Initial draft underspecified source-label custody; resolved by adding the Data steward actor and the source-needed hold path in OPS-001. | resolved |

Fixed-point note: one actionable finding (source-label custody under-specified) was
raised and applied. No unresolved critical or major finding remains.

## Open Questions

| ID | Question | Disposition |
|---|---|---|
| OQ-001 | What is the exact dimension pool and its definitions? | Defer to `REQUIREMENTS.md` and `SPECIFICATION_BASELINE.md`. |
| OQ-002 | Which data sources become the first `EVID-*` sources, and at what cadence? | Defer to `data/sources.md` and `VERIFICATION.md`. |
| OQ-003 | What reliability basis (shared-track dispatch vs dedicated) anchors OTP scoring? | Defer to `SPECIFICATION_BASELINE.md`. |

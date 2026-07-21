# Pulse 04: SPECIFICATION_BASELINE

Settled. Authored `docs/vtrace/SPECIFICATION_BASELINE.md` (target/provisional):
the rail dimension pool `DIM-01..13`, the reliability basis (SPEC-RB-01/02:
dedicated vs shared host track), the **T1–T4 tier model** (HSR Spine / Intercity
Trunk / Regional Connector / Branch-Feeder) with trip-time/frequency/reliability/
access SLA terms, controlled specs `SPEC-001..012`, public contracts `IF-001..004`,
package allocation, and the spec gate (`pass_with_risk`).

Role-review fixed point (real `.roles`): one finding applied — shared-track OTP
(DIM-06) risked reading as measured, resolved via SPEC-RB-02 (labelled proxy) +
SPEC-UNK-002; benefit-cost (DIM-11) and tier-SLA conformance (DIM-13) default
labels set to `heuristic`. Pool weights and SLA thresholds explicitly provisional.

Validation: `proof check .`, `git diff --check`. Next: TRACE.

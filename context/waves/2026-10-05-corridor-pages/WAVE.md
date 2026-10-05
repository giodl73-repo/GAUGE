# WP-007: Browser frequency explorer

Status: accepted for implementation (2026-10-05).
Parent: REQ-001 pipeline, REQ-003 sources, REQ-007 reliability basis, REQ-010 promotion boundary; native DIM-07 scoring and gap interfaces.

Reader task: choose one of 12 committed historical corridor entries, adjust hypothetical round trips/day (0-32), adjust the provisional DIM-07 adequacy bar (0-10), compare baseline and scenario corpus membership below the bar.
Use existing Rust corpus parsing and gap detection in a new gauge-web adapter compiled to WASM. No network data acquisition or TRACKER path dependence. Bundle committed corpus with include_str; preserve source IDs, historical 2023/2024 counts, stored baseline scores and tier metadata. New scenario frequency transform is the existing documented clamp(trips*10/16,0,10), rounded to one decimal like corpus scores. Shared helper belongs to gauge-score.
All changes are explicitly hypothetical; changing frequency does not recalculate tier, timetable feasibility, track capacity, ridership, costs, reliability or modal shift. Other 12 dimensions remain unassessed. Do not manufacture a full-pipeline null result from frequency adequacy.
Interface: bounded JSON request (8KB), selected corpus index, hypothetical frequency, adequacy bar; bounded JSON response containing baseline and scenario, source IDs and score basis. Worker handles errors/latest request; URLs reproduce scenarios; download JSON and source/artifact links; mobile keyboard controls.
Verification: format, strict workspace clippy, all native tests, baseline eight-below fixture, boundary/invalid-input tests, actual WASM browser interaction/share/export/failure/mobile tests, review and Pages CI deployment.

## Role Review Notes

| Role | Disposition | Contract / resolved finding |
|---|---|---|
| Rail Planner | pass | Whole corpus remains visible; frequency only, no network superiority claim. |
| Rail Civil Engineer | pass | No speeds, geometry or buildability claims. |
| Operations Officer | resolved | Hypothetical frequency must not be called an operable timetable. |
| Transport Economist | pass | No ridership, spending or benefit-cost outputs. |
| Climate/Modal Advocate | defer | Modal shift/electrification require additional sourced dimensions in a later WP. |
| Equity/Access Advocate | resolved | Low-frequency corridors remain visible; adequacy is not an access-equity assessment. |
| Freight-Host Realist | resolved | More trains are a sensitivity input, not evidence of available host-track capacity. |
| Citation Auditor | resolved | Explicit historical date/source registry; do not label count as current live service. |
| Scope Keeper | resolved | Tier labels remain metadata; no full tier-SLA conformance output. |
| Numeracy Checker | resolved | Round trips/day, clamped 0-10 score, one-decimal rounding and 7.0 provisional bar stated. |

Fixed point: no unresolved critical/major actionable finding. All resolved contract changes incorporated above before implementation. This is a product adapter acceptance, not promotion of a rail proposal.

## Pulse evidence

Implementation complete: native/WASM adapter, worker, URL sharing, JSON/source/artifact downloads, responsive page, Pages workflow.41 full-workspace Rust tests, strict workspace clippy, six actual-WASM Chromium tests passed. Final artifact 253,822 bytes. Review and deployed CI are final gates.

Review fixes: stable historical cache for rapid corridor switching; Reset clears pre-initialization shared input; exact native score threshold override prevents floating-point disagreement with membership. Native and browser regressions passed for each.

Implementation pulse complete. Final built-in Codex review clean after all three findings were fixed and regression-tested. Publication CI remains the final gate.

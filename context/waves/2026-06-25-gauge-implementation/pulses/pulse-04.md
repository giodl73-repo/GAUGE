# Pulse 04: WP-004 `gauge-tier` tier classification + SLA conformance

Status: pending (blocked by WP-003). Executes WP-004.

Classify each corridor into T1–T4, attach tier SLA terms, compute DIM-13
conformance (naming the dispatch basis, REQ-007), and emit tier-SLA gaps. Creates
`crates/gauge-tier/` and a `tiers.toml` SLA record (IF-004), values provisional.

Parent: REQ-014/015 · SPEC-011/012 · DIM-13 · IF-004 · PKG-004 · CR-004/007.

Exit: `cargo test -p gauge-tier` green (classification, conforming corridor,
shortfall → tier-SLA gap). On completion: VER-014/015 → passed; TRACE REQ-014/015 →
implemented; WP-004 → done; unblock WP-005.

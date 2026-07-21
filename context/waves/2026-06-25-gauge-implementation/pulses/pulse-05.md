# Pulse 05: WP-005 `gauge-gap` gap analysis

Status: pending (blocked by WP-004). Executes WP-005.

Plot scored corridors in dimension space, find under-served regions, and record a
car/air-dominant market as a labelled null result (REQ-008). Integrates tier-SLA
gaps from `gauge-tier`. Creates `crates/gauge-gap/`.

Parent: REQ-008 · SPEC-006 · PKG-005 · CR-004.

Exit: `cargo test -p gauge-gap` green (one found gap, one null-result case). On
completion: VER-008 → passed; TRACE REQ-008 → implemented; WP-005 → done; unblock
WP-006.

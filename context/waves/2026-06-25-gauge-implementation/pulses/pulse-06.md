# Pulse 06: WP-006 `gauge-cli` orchestration

Status: pending (blocked by WP-005). Executes WP-006.

`gauge` CLI exposing `corpus`, `score`, `tier-sla`, and `gap` subcommands that
drive the pipeline and emit reproducible artifacts with labels preserved. Creates
`crates/gauge-cli/`. No VTRACE subcommands — product/process split per
IMPLEMENTATION_PLAN.

Parent: REQ-001 · IF-006 · PKG-006 · CR-003/006/008.

Exit: `cargo run -p gauge-cli -- --help` lists product subcommands; end-to-end run
on a seed corpus regenerates artifacts deterministically; `cargo test --workspace`
green. On completion: VER-001 → passed; TRACE REQ-001 → implemented; WP-006 → done;
pipeline end-to-end; L2 readiness review on the seed corpus.

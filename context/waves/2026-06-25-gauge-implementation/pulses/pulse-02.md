# Pulse 02: WP-002 `gauge-corpus` model, schema, sources, labels

Status: pending (blocked by WP-001). Executes WP-002.

Represent one corridor as a labelled, sourced, tiered corpus entry, and
hold/reject unidentified or uncited rows. Creates `crates/gauge-corpus/`,
`corpus/SCHEMA.md` (IF-001), and `data/sources.md` (IF-002).

Parent: REQ-001/002/003/005 · SPEC-002/003/009 · IF-001/002 · PKG-002 · CR-003/004/007.

Exit: `cargo test -p gauge-corpus` green (missing-id reject, uncited quantity held,
label preservation); `proof check .` clean. On completion: VER-001/002/003 → passed;
TRACE REQ-001/002/003/005 → implemented; WP-002 → done; unblock WP-003.

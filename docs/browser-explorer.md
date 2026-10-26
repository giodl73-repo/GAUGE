# Browser frequency explorer

GAUGE owns gauge-web; it uses native gauge-corpus parsing, gauge-score frequency transform and gauge-gap classification. No cross-product dependency, filesystem or network data acquisition at runtime. Twelve committed corpus documents are embedded in WASM, retaining source IDs and historical dates. A changed frequency is labelled hypothetical; its historical source remains separately named.

Input JSON is bounded to 8 KB; selected index 0-11, round trips/day 0-32, score bar 0-10. The documented transform clamps trips*10/16 to 0-10 and rounds to one decimal like corpus entries. Baseline stored scores are preserved. The selected corridor alone changes. No trip time, full SLA, ridership, cost, carbon, capacity or access-equity finding is computed. Other twelve dimensions are unassessed.

Reproduce: cargo test --workspace --locked; cargo clippy --workspace --all-targets -- -D warnings; npm ci; cargo install wasm-bindgen-cli --version 0.2.127 --locked; python tools/build-pages.py; npx playwright install chromium; npm run test:pages. Windows can set GAUGE_BROWSER_PATH to installed Chromium. Rust 1.95.0 and wasm-bindgen 0.2.127 are pinned.

Pages build output stays within dist and is capped at 5 MB. Master deploys after native/browser checks. Pull requests build without deployment. Site offers JSON, Rust source and WASM downloads, source registry and mixed-license notice. Browser JavaScript presents Rust outputs; a worker keeps the UI responsive and drops stale responses.

WP-007 acceptance and all ten role dispositions are in docs/vtrace/WORK_PACKAGES.md and context/waves/2026-10-05-corridor-pages/WAVE.md.

## Portfolio scenarios (v2)

Edits are retained by corpus slug when switching corridors. The optional `portfolio` URL parameter is a JSON map of slug to hypothetical round trips/day; it overrides the legacy selected-corridor value. Existing `corridor`, `round_trips`, `bar` links remain supported. The 12-trip preset raises low frequencies while preserving higher ones. Reset clears all edits. Export includes the complete map and both runs. Historical sources and all unassessed dimensions remain unchanged.

## Source-custody correction (WP-009)

All twelve original frequency quantities are source-needed. Existing v2 inputs,
share links and numerical fields remain; exports add each original quantity's
actual `historical_evidence_label`, false `historical_primary_source_verified`
and a versioned `evidence` boundary. Eleven service/representativeness/cost
observations stay explicit null; seven authorities stay false. Twelve other
native dimensions remain unassessed even if every model score clears the bar.
The average is a frequency proxy, not measured span, spacing or operating days.

The static audit remains available without WASM, with twelve exact corpus copies,
corrected and superseded-original findings, registry, audit note and hashes.
No primary source is verified by a derivative-file hash. `source-audit.json`
distinguishes absent original source locators/hashes from retained record custody.
The native `gauge-browser` binary and actual WASM match canonical JSON on four
fixtures; the build also compares every pre-existing v2 field with the original
deployed-WASM results before this additive metadata correction. No score/gap
algorithm, numeric value, tier or provisional threshold changed.

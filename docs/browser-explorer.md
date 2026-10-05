# Browser frequency explorer

GAUGE owns gauge-web; it uses native gauge-corpus parsing, gauge-score frequency transform and gauge-gap classification. No cross-product dependency, filesystem or network data acquisition at runtime. Twelve committed corpus documents are embedded in WASM, retaining source IDs and historical dates. A changed frequency is labelled hypothetical; its historical source remains separately named.

Input JSON is bounded to 8 KB; selected index 0-11, round trips/day 0-32, score bar 0-10. The documented transform clamps trips*10/16 to 0-10 and rounds to one decimal like corpus entries. Baseline stored scores are preserved. The selected corridor alone changes. No trip time, full SLA, ridership, cost, carbon, capacity or access-equity finding is computed. Other twelve dimensions are unassessed.

Reproduce: cargo test --workspace --locked; cargo clippy --workspace --all-targets -- -D warnings; npm ci; cargo install wasm-bindgen-cli --version 0.2.127 --locked; python tools/build-pages.py; npx playwright install chromium; npm run test:pages. Windows can set GAUGE_BROWSER_PATH to installed Chromium. Rust 1.95.0 and wasm-bindgen 0.2.127 are pinned.

Pages build output stays within dist and is capped at 5 MB. Master deploys after native/browser checks. Pull requests build without deployment. Site offers JSON, Rust source and WASM downloads, source registry and mixed-license notice. Browser JavaScript presents Rust outputs; a worker keeps the UI responsive and drops stale responses.

WP-007 acceptance and all ten role dispositions are in docs/vtrace/WORK_PACKAGES.md and context/waves/2026-10-05-corridor-pages/WAVE.md.

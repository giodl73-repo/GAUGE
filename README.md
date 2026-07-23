# GAUGE

**Rail 2.0 — evidence-gated passenger-rail network analysis and conceptual design.**

**Tracks connect places. Service promises make rail usable.**

GAUGE asks whether a corridor offers credible trip time, frequency, reliability,
and connections—not merely whether trains exist. It scores existing US
passenger-rail corridors, classifies service tiers, finds measurable gaps, and
preserves the possibility that driving or flying is the stronger answer.

**Series:** [Applied Systems](https://github.com/giodl73-repo/giodl73-repo/blob/main/series/applied-systems.md)

> GAUGE is a research and conceptual-design project. It is not an engineering
> study, environmental review, procurement plan, timetable, or advocacy brief,
> and it claims no FRA, Amtrak, state-DOT, or host-railroad endorsement.

## Use GAUGE

GAUGE is public and open to use as a reference model, cited rail-service
finding, diagnostic pattern, review discipline, or local adaptation starting
point.

If you want to apply it to a corridor, region, state rail plan, station-access
problem, or passenger-rail service question, start with
[`docs/adoption/README.md`](docs/adoption/README.md). It lays out safe reuse,
first adaptation steps, contribution targets, and claim boundaries.

## Why this matters

Passenger rail arguments often jump from enthusiasm to a map. GAUGE inserts the
missing middle: a cited corpus, a common scoring instrument, explicit service
tiers, and gap evidence that can survive host-railroad, operations, engineering,
economic, access, and climate review.

The transferable principle is simple: **a mode earns investment by meeting a
service promise, not by winning a narrative.**

## What is implemented

| Crate | Responsibility |
|---|---|
| `gauge-network` | Passenger-rail network elements and relationships. |
| `gauge-corpus` | Evidence-labelled corridor corpus parsing and validation. |
| `gauge-score` | DIM-01..13 score artifacts. |
| `gauge-tier` | Tier-SLA classification and shortfall reporting. |
| `gauge-gap` | Gap analysis, dispersion signals, and explicit null results. |
| `gauge-cli` | CLI front door for corpus, score, tier-SLA, and gap commands. |

## Evidence

The current
[frequency and span-of-service analysis](docs/findings/2026-06-frequency-span-of-service.md)
covers 12 US corridors. Eight fall below the declared bar, producing a systemic
rather than tail-only gap classification.

That is evidence about the tested corpus, not a national construction mandate.

## Quick start

```powershell
cargo run -p gauge-cli -- corpus --input corpus
cargo run -p gauge-cli -- gap --input corpus
cargo test --workspace
```

## Method

```text
CORPUS -> SCORE -> TIER-SLA -> GAP -> CONCEPT -> REVIEW -> DESIGN
```

Every promoted claim keeps its source, evidence label, and comparison basis.
A rigorous null result remains a valid result.

## Documentation

- [`PRODUCT_PLAN.md`](PRODUCT_PLAN.md) — scope, product shape, and next work.
- [`docs/adoption/`](docs/adoption) — open reuse, local adaptation, and review path.
- [`docs/vtrace/`](docs/vtrace) — VTRACE requirements, architecture, trace, and verification.
- [`context/waves/`](context/waves) — repo-local execution history.
- [`.roles/ROLE.md`](.roles/ROLE.md) — adversarial review panel.

## License

MIT. See [`LICENSE`](LICENSE).

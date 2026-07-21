# GAUGE

**Rail 2.0 — evidence-gated passenger-rail network analysis and conceptual design.**

GAUGE scores existing US passenger-rail corridors, classifies service tiers,
finds measurable gaps, and produces bounded design evidence. It applies the
same corpus-to-gap method as ROUTE without assuming that every corridor should
be rail or that a positive build case must exist.

> GAUGE is a research and conceptual-design project. It is not an engineering
> study, environmental review, procurement plan, timetable, or advocacy brief,
> and it claims no FRA, Amtrak, state-DOT, or host-railroad endorsement.

## What is implemented

| Crate | Responsibility |
|---|---|
| `gauge-network` | Passenger-rail network elements and relationships. |
| `gauge-corpus` | Evidence-labelled corridor corpus parsing and validation. |
| `gauge-score` | DIM-01..13 score artifacts. |
| `gauge-tier` | Tier-SLA classification and shortfall reporting. |
| `gauge-gap` | Gap analysis, dispersion signals, and explicit null results. |
| `gauge-cli` | CLI front door for corpus, score, tier-SLA, and gap commands. |

The current cited frequency run covers 12 US corridors. Eight fall below the
declared bar, producing a systemic rather than tail-only gap classification.
That is evidence about the tested corpus, not a national construction mandate.

## Quick start

```powershell
cargo run -p gauge-cli -- --help
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
- [`docs/vtrace/`](docs/vtrace) — VTRACE requirements, architecture, trace, and verification.
- [`context/waves/`](context/waves) — repo-local execution history.
- [`.roles/ROLE.md`](.roles/ROLE.md) — adversarial review panel.

## License

MIT. See [`LICENSE`](LICENSE).

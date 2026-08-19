# GAUGE

**Rail 2.0 — evidence-gated passenger-rail network analysis and conceptual design.**

**Tracks connect places. Service promises make rail usable.**

GAUGE asks whether a corridor offers credible trip time, frequency, reliability,
and connections—not merely whether trains exist. It scores existing US
passenger-rail corridors, classifies service tiers, finds measurable gaps, and
preserves the possibility that driving or flying is the stronger answer.

**Series:** [Applied Systems](https://github.com/giodl73-repo/giodl73-repo/blob/main/series/applied-systems.md)

## Show someone (start here)

| Audience | Path | Time |
|---|---|---|
| **Planner / researcher** | [SHOWCASE.md](SHOWCASE.md) → [frequency/span finding](docs/findings/2026-06-frequency-span-of-service.md) | 15–25 min |
| **CLI implementer** | [SHOWCASE.md](SHOWCASE.md) → `cargo run -p gauge-cli -- gap --input corpus` | 10–20 min |

Research lab only — not an engineering study, timetable, or FRA/Amtrak endorsement.
Null results (drive/fly stronger) are valid.

## Infrastructure 2.0 family

GAUGE is one domain implementation of a shared evidence-first method:

```text
PUBLIC SOURCES → CORPUS → SCORE → SERVICE PROMISE → GAP MAP
                                                     ↓
                                      CONCEPT → REVIEW → DESIGN
```

| Lane | Repositories |
|------|--------------|
| Movement | [ROUTE](https://github.com/giodl73-repo/ROUTE), [GAUGE](https://github.com/giodl73-repo/GAUGE), [TARMAC](https://github.com/giodl73-repo/TARMAC), [HARBOR](https://github.com/giodl73-repo/HARBOR) |
| Lifelines | [PYLON](https://github.com/giodl73-repo/PYLON), [PACKET](https://github.com/giodl73-repo/PACKET), [BASIN](https://github.com/giodl73-repo/BASIN), [DRAIN](https://github.com/giodl73-repo/DRAIN) |
| Public access | [SHIELD](https://github.com/giodl73-repo/SHIELD), [SLATE](https://github.com/giodl73-repo/SLATE) |
| Civic boundaries | [ZONES](https://github.com/giodl73-repo/ZONES) |

The family shares evidence labels, explicit scale and demand bases, T1–T4
service promises where meaningful, adversarial review, and acceptance of a
rigorous null result. Each repository owns its domain semantics and safety
boundary.

> GAUGE is a research and conceptual-design project. It is not an engineering
> study, environmental review, procurement plan, timetable, or advocacy brief,
> and it claims no FRA, Amtrak, state-DOT, or host-railroad endorsement.

## Use GAUGE

GAUGE is public and open to use as a reference model, cited rail-service
finding, diagnostic pattern, review discipline, or local adaptation starting
point.

### Reuse boundary

GAUGE is currently a specialist passenger-rail analysis product, not a
supported cross-repository library. Its network model, evidence taxonomy,
DIM-01..13 scoring, service tiers, gap policy, and CLI remain product-local; no
portfolio repository pins a `gauge-*` crate or owns compatibility proof.

GAUGE adapted early PYLON scoring implementation, but copied source is not a
stable provider contract. Infrastructure 2.0 siblings share a method while each
owns its domain semantics. Direct GAUGE reuse requires a named downstream
consumer, a bounded versioned surface, and consumer-owned compatibility tests.

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

GAUGE uses separate licenses for software and content. Source code,
executable scripts, tests, configuration, and ordinary software
documentation are MIT-licensed (copyright Gio Della-Libera). Original
non-software content is licensed CC BY-NC 4.0 (copyright Gio Della-Libera);
commercial use of that content requires separate written permission.
Third-party material remains under its own terms.
See [LICENSE](./LICENSE) for the complete notice.

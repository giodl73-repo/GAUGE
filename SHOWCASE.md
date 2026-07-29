# GAUGE Showcase — Rail 2.0

**Who this is for:** someone you would hand the repo to for 15–30 minutes —
a **rail / mobility researcher** asking whether corridors meet trip-time,
frequency, reliability, and connection promises, or a **CLI implementer**
running corpus → gap.

**Posture:** research and conceptual-design lab. **Not** an engineering study,
environmental review, procurement plan, timetable, or FRA / Amtrak / state-DOT /
host-railroad endorsement. A rigorous null (drive or fly is stronger) is a valid
result.

| Audience | Open this first | Time |
|---|---|---|
| Planner / researcher | [Frequency / span finding](docs/findings/2026-06-frequency-span-of-service.md) | 15–25 min |
| CLI implementer | README crate table + gap command | 10–20 min |
| Local adapter | [Adoption guide](docs/adoption/README.md) | 15–25 min |

## One-minute pitch

**Tracks connect places. Service promises make rail usable.**

GAUGE scores existing US passenger-rail corridors, classifies service tiers,
finds measurable gaps, and refuses to treat “a train exists” as proof of useful
service.

```text
CORPUS -> SCORE -> TIER-SLA -> GAP -> CONCEPT -> REVIEW -> DESIGN
```

## Two doors

### A. Planner / researcher path

**Question GAUGE answers well:** *Which corridors fail credible frequency,
span, trip-time, reliability, or connectivity promises on a cited corpus?*

| Step | What to look at | Why |
|---|---|---|
| 1 | README boundary callout | No advocacy-brief framing |
| 2 | [2026-06 frequency / span finding](docs/findings/2026-06-frequency-span-of-service.md) | 12-corridor cited run; 8 below bar → systemic gap class |
| 3 | [Adoption / local adaptation](docs/adoption/README.md) | Reuse without overclaim |
| 4 | Reproduce | `cargo run -p gauge-cli -- gap --input corpus` |

**Headline (cite with scope):** on the documented 12-corridor DIM-07 frequency
run, eight corridors fall below the declared bar — evidence about **that corpus
and dimension**, not a national construction mandate.

**Do not say:** build HSR here, Amtrak timetable replacement, or host-railroad
agreement.

### B. CLI implementer path

| Crate | Responsibility |
|---|---|
| `gauge-network` | Passenger-rail network elements |
| `gauge-corpus` | Evidence-labelled corridor corpus |
| `gauge-score` | DIM-01..13 score artifacts |
| `gauge-tier` | Tier-SLA classification |
| `gauge-gap` | Gaps, dispersion signals, nulls |
| `gauge-cli` | Corpus / score / tier / gap front door |

```powershell
cargo run -p gauge-cli -- corpus --input corpus
cargo run -p gauge-cli -- gap --input corpus
cargo test --workspace
```

## Claim packet (this showcase)

| Field | Value |
|---|---|
| Claim text | GAUGE can be shown as Rail 2.0 service-promise scoring with at least one cited frequency finding and a reusable CLI pipeline. |
| Audience | Mobility researchers; CLI implementers. |
| Evidence | README; 2026-06 finding; adoption docs; CLI reproduce path. |
| Validation | Finding-scoped corpus run; not nationwide operations certification. |
| Limitations | Single-dimension public finding is illustrative of method depth, not full multi-DIM national scoreboard completeness. |
| Non-claims | Engineering design, NEPA, procurement, official ridership forecasts, endorsements. |

## Where not to start

| Avoid… | Why |
|---|---|
| Map-first advocacy deck | Skips the evidence middle |
| Treating null as failure | Null is a first-class outcome |

## Related

- Family hub: [`../README.md`](../README.md)
- Product plan: [`PRODUCT_PLAN.md`](PRODUCT_PLAN.md)

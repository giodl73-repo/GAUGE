# Frequency / Span of Service: when the "tail" is the whole network

Date: 2026-06
Rubric: GAUGE v0 (`UNDER_SERVED_THRESHOLD = 7.0`)
Corpus: `corpus/us-*.md` (12 US passenger-rail corridors)
Dimension assessed: DIM-07 Frequency / Span of Service (only)
Reproduce: `cargo run -p gauge-cli -- gap --input corpus`

## Question

The tail/dispersion gap signal was validated on a spread-out distribution
(PACKET broadband, ~half a deficient minority) and a concentrated-high one
(HARBOR channel depth, a 2-of-8 minority tail). What does it do on a **bimodal,
majority-deficit** distribution — US intercity rail, where a few corridors are
frequent and most run once a day or less?

## Data

DIM-07 scores service frequency. Transform: `clamp(daily_round_trips * 10/16, 0, 10)`,
so ~16 round trips/day (roughly hourly over a useful span) maps to the 10.0
ceiling and the 7.0 adequacy bar corresponds to ~11 round trips/day. Frequency is
the only scored dimension; every count is a cited timetable figure
(`wikipedia-amtrak-routes`, 2023/2024). Route size/length is not scored.

| Corridor | Round trips/day | DIM-07 |
|---|---|---|
| Northeast Regional | 18 | 10.0 |
| Acela | 16 | 10.0 |
| Keystone Service | 12 | 7.5 |
| Pacific Surfliner | 12 | 7.5 |
| Capitol Corridor | 9 | 5.6 |
| Hiawatha | 7 | 4.4 |
| Cascades | 4 | 2.5 |
| Wolverine | 3 | 1.9 |
| Empire Builder | 1 | 0.6 |
| California Zephyr | 1 | 0.6 |
| Crescent | 1 | 0.6 |
| Sunset Limited | 0.4 (3/week) | 0.3 |

DIM-07: mean 4.29, min 0.3, bottom-quartile (3 lowest) mean 0.5.
**8 of 12 corridors (67%) fall below the 7.0 bar.**

## Result

| Detector | Logic | Fires? | Names | Verdict |
|---|---|---|---|---|
| Mean @ 5.0 (PACKET-style) | corpus mean < bar | Yes (4.29) | (dimension-level) | Correct: the deficit is **systemic** |
| Min @ 7.0 (GAUGE `UnderServedRegion`) | lowest < bar | Yes (0.3) | **all 12 corridors** | Fires, but blames the 4 adequate corridors too |
| Tail @ 7.0 (`TailRegion`) | bottom-quartile mean < bar | Yes (0.5) | **the 8 sub-bar corridors** | Correct membership, but see below |

The tail detector did the right thing on *membership*: it named exactly the 8
corridors below the bar and correctly excluded the four adequate ones (the two
NEC services, Keystone, Surfliner) that the min detector wrongly swept in.

## The new finding: tail membership ≠ a tail

But naming 8 of 12 is not a "tail" — it is a **majority**. The bottom-quartile
*trigger* is distribution-shape-agnostic (it fired here exactly as it did on
HARBOR's 2-of-8 minority), yet the *interpretation* must differ:

| Run | Distribution | Share below bar | Honest read |
|---|---|---|---|
| HARBOR (channel depth) | concentrated-high | 2 of 8 (25%) | genuine **tail** — target the named minority |
| PACKET (broadband) | spread | ~half | mixed |
| GAUGE (rail frequency) | bimodal | 8 of 12 (67%) | **systemic** — the network, not a tail |

A 25%-below result says "fix these two ports." A 67%-below result says "most of
the US intercity network is below a useful-frequency bar" — which is precisely
GAUGE's Rail-2.0 thesis. Treating the latter as a "tail" understates it: the
recommendation flips from *target the named minority* to *the whole class needs
the upgrade*.

## Recommendation

The tail trigger is a good universal gate, but a single number is missing to
read it correctly: the **share of scored entries below threshold**. Add a
`share_below_threshold` (and/or a `systemic` vs `tail` classification) to the
dispersion signal across the portfolio:

- share ≤ ~⅓ → genuine tail; act on `member_ids`.
- share ≥ ~½ → systemic deficit; the dimension-level region is the headline and
  the member list is "everyone but the adequate few."

This is the same flywheel that produced the tail signal itself: a cited run
exposed a gap in the method, and the fix is a small, propagable signal.

**Update (implemented):** GAUGE's `gauge-gap` now computes the share below
threshold and reclassifies the dispersion region as `GapSource::SystemicRegion`
(id `systemic-<dim>`) when the share reaches `SYSTEMIC_SHARE` (0.5), keeping
`TailRegion` for genuine minorities. Re-running this corpus now emits
`systemic-DIM-07` with reason "8 of 12 scored entries (67%) fall below the bar,
classified as a systemic deficit". Proposed next step — propagate the
share/classification to the other seven gap crates.

## Honesty notes

- Frequency is the only scored dimension; 12 of 13 dimensions are reported as
  explicit `EmptyRegion` coverage gaps, not passing scores.
- NEC counts are per-brand (Acela and Northeast Regional are listed separately);
  combined NEC frequency is higher than either row.
- Sunset Limited's 0.4/day is `3 round trips per week ÷ 7`, labelled as such.
- The 16-round-trips → 10.0 ceiling is a documented modeling choice; it sets the
  ~11/day adequacy bar and is not a cited standard.

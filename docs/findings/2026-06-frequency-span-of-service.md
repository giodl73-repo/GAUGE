# Historical frequency-score illustration: majority versus tail

Original model date: 2026-06. Source-custody correction: 2026-10-26.
Scope: twelve retained US corridor/brand model entries; no national inference.
Rubric: GAUGE v0, provisional DIM-07 bar 7.0.
Dimension assessed: DIM-07 Frequency / Span of Service (only), through a frequency
proxy; span is unobserved. Frequency is the only scored dimension.
Reproduce: `cargo run -p gauge-cli -- gap --input corpus`.

## Question

How does the native dispersion detector distinguish a low-score majority from a
minority tail on this selected model distribution? This is a detector illustration,
not a measured service-span or nationwide passenger-rail finding.

## Retained model inputs

All twelve physical frequency quantities are source-needed after the custody
audit. Their broad source ID is preserved, but original timetable bytes and
record-level route/date/segment/day locators are absent. This does not prove the
values wrong; it prevents treating them as verified timetable observations.
The 2023/2024 context does not establish a per-record window or present service.

The unchanged transform clamps round trips/day times 10/16 to 0–10 and rounds to
one decimal. The 7.0 bar is provisional, not a cited service standard. At integer
daily inputs, 11 maps to 6.9 and 12 to 7.5. Frequency alone does not establish
service span, actual departure spacing, operating days or endpoint coverage.

| Retained entry | Model round trips/day | Stored DIM-07 score |
|---|---:|---:|
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
| Sunset Limited | 0.4 | 0.3 |

The Sunset value is a rounded historical model claim, not exactly 3/7 and not a
daily operating pattern. The old three-per-week description also needs a source.
NEC brand entries do not establish whole-endpoint, every-segment coverage.

## Scoped result

Eight of twelve retained scores fall below 7.0. Their stored-score mean is about
4.29, minimum 0.3 and bottom-three mean 0.5. The minimum alarm names all twelve
scored entries. The dispersion detector names the eight sub-bar entries and
classifies the distribution as `SystemicRegion` because their share exceeds the
native 0.5 classification threshold. This is a property of these inputs.

The same native classifier can name a minority `TailRegion` under different
hypothetical inputs. Neither detector membership is an operational intervention
target list. This selected corpus does not establish national representativeness,
actual service deficits, useful all-day service or a construction mandate.

## What remains held

DIM-07 is the only scored dimension, and even here the score represents a
frequency proxy rather than a measured span. 12 of 13 dimensions are reported as explicit `EmptyRegion` coverage gaps
by the native gap CLI; twelve other dimensions remain unassessed. Tier metadata does not establish SLA conformance. Clearing model
scores cannot supply missing source, timetable, dispatch, host capacity,
reliability, demand, access, cost, modal-shift or savings evidence.

The [source audit](https://giodl73-repo.github.io/GAUGE/evidence/source-audit.md)
names the next source-to-score review gate. The
[superseded original](https://giodl73-repo.github.io/GAUGE/evidence/original-finding.md)
is preserved byte for byte. Its national-network and blanket-upgrade inferences
are withdrawn; no original source, score or threshold is silently replaced.

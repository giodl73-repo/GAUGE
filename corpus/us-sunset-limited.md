---
id: corridor:sunset-limited
type: corridor
termini: [new-orleans, los-angeles]
states: [LA, TX, NM, AZ, CA]
tier: T4
---

# Sunset Limited

DIM-07 is Frequency / Span of Service. Score transform: `clamp(daily_round_trips * 10/16, 0, 10)`,
so 16 round trips/day maps to the 10.0 ceiling. The 7.0 bar is provisional; integer
11/day maps to 6.9 and 12/day to 7.5. This score does not measure actual span or spacing;
the 0.4/day quantity is a rounded model claim requiring original timetable
sources. It is not exactly 3/7, and neither the rounded average nor the retained
three-per-week description establishes a verified operating-day pattern.

score: DIM-07 | 0.3
quantity: 0.4 | round-trips-per-day | source-needed | wikipedia-amtrak-routes
Custody correction (2026-10-26): the retained frequency quantity has no original
timetable bytes or route/date/segment/day-specific locator in this repository.
Its value and score are unchanged; measured span and operating days are unknown.
See `data/source-audit.md`.

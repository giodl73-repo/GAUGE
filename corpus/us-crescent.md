---
id: corridor:crescent
type: corridor
termini: [new-york, new-orleans]
states: [NY, NJ, PA, MD, DC, VA, NC, SC, GA, AL, MS, LA]
tier: T4
---

# Crescent

DIM-07 is Frequency / Span of Service. Score transform: `clamp(daily_round_trips * 10/16, 0, 10)`,
so 16 round trips/day maps to the 10.0 ceiling. The 7.0 bar is provisional; integer
11/day maps to 6.9 and 12/day to 7.5. This score does not measure actual span or spacing;
counts are retained model inputs requiring original timetable sources.

score: DIM-07 | 0.6
quantity: 1 | round-trips-per-day | source-needed | wikipedia-amtrak-routes
Custody correction (2026-10-26): the retained frequency quantity has no original
timetable bytes or route/date/segment/day-specific locator in this repository.
Its value and score are unchanged; measured span and operating days are unknown.
See `data/source-audit.md`.

---
id: corridor:wolverine
type: corridor
termini: [chicago, pontiac]
states: [IL, IN, MI]
tier: T3
---

# Wolverine

DIM-07 is Frequency / Span of Service. Score transform: `clamp(daily_round_trips * 10/16, 0, 10)`,
so 16 round trips/day maps to the 10.0 ceiling. The 7.0 bar is provisional; integer
11/day maps to 6.9 and 12/day to 7.5. This score does not measure actual span or spacing;
counts are retained model inputs requiring original timetable sources.

score: DIM-07 | 1.9
quantity: 3 | round-trips-per-day | source-needed | wikipedia-amtrak-routes
Custody correction (2026-10-26): the retained frequency quantity has no original
timetable bytes or route/date/segment/day-specific locator in this repository.
Its value and score are unchanged; measured span and operating days are unknown.
See `data/source-audit.md`.

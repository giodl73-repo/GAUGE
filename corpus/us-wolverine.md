---
id: corridor:wolverine
type: corridor
termini: [chicago, pontiac]
states: [IL, IN, MI]
tier: T3
---

# Wolverine

DIM-07 is Frequency / Span of Service. Score transform: `clamp(daily_round_trips * 10/16, 0, 10)`,
so ~16 round trips/day (roughly hourly over a useful span) maps to the 10.0 ceiling and the
7.0 adequacy bar corresponds to ~11 round trips/day. Frequency is the only scored dimension here;
all figures are cited timetable counts.

score: DIM-07 | 1.9
quantity: 3 | round-trips-per-day | implemented | wikipedia-amtrak-routes
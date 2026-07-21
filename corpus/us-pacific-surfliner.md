---
id: corridor:pacific-surfliner
type: corridor
termini: [san-luis-obispo, san-diego]
states: [CA]
tier: T2
---

# Pacific Surfliner

DIM-07 is Frequency / Span of Service. Score transform: `clamp(daily_round_trips * 10/16, 0, 10)`,
so ~16 round trips/day (roughly hourly over a useful span) maps to the 10.0 ceiling and the
7.0 adequacy bar corresponds to ~11 round trips/day. Frequency is the only scored dimension here;
all figures are cited timetable counts.

score: DIM-07 | 7.5
quantity: 12 | round-trips-per-day | implemented | wikipedia-amtrak-routes
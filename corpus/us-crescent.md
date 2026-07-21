---
id: corridor:crescent
type: corridor
termini: [new-york, new-orleans]
states: [NY, NJ, PA, MD, DC, VA, NC, SC, GA, AL, MS, LA]
tier: T4
---

# Crescent

DIM-07 is Frequency / Span of Service. Score transform: `clamp(daily_round_trips * 10/16, 0, 10)`,
so ~16 round trips/day (roughly hourly over a useful span) maps to the 10.0 ceiling and the
7.0 adequacy bar corresponds to ~11 round trips/day. Frequency is the only scored dimension here;
all figures are cited timetable counts.

score: DIM-07 | 0.6
quantity: 1 | round-trips-per-day | implemented | wikipedia-amtrak-routes
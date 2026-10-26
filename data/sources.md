# Source Registry

Every cited corpus quantity points to a source id in this registry. Entries can
be expanded as the corpus grows; a source id is stable once cited. Candidate
public sources for the passenger-rail corpus include the BTS National
Transportation Atlas, FRA route and ridership data, Amtrak route timetables and
performance reports, and Census/ACS commute data.

| Source ID | Label | Citation | Notes |
|---|---|---|---|
| `SRC-SEED-001` | source-needed | GAUGE seed placeholder | Used only to document the registry shape until real public sources are admitted. |
| `wikipedia-amtrak-routes` | source-needed | Wikipedia, "List of Amtrak routes" (2023/2024 schedules), corroborated against Amtrak published timetables | Daily round-trip counts per route used for DIM-07 Frequency / Span of Service. Long-distance routes run 1 round trip/day; Sunset Limited runs 3 round trips/week (0.4/day). NEC counts are per-brand (Acela ~16 weekday round trips; Northeast Regional ~18). |

## Source-custody correction — 2026-10-26

The historical registry wording above records a broad reference and a claimed
corroboration, not retained original timetable evidence. No exact article revision,
original schedule bytes or record-level locators substantiate the twelve model
quantities here. They are all source-needed; IDs and values are retained.
See [source-audit.md](source-audit.md). The 0.4/day value is rounded, not exactly
3/7 and not proof of service every day. Counts do not establish measured span,
segment coverage, operating days or national representativeness.

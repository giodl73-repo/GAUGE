# Frozen Seattle-Portland timetable source

Document: Amtrak Cascades-branded two-page schedule, effective December 11, 2023.
Retrieved from the URL in custody.json; ODOT-named S3 host ownership has not been
independently attested. Branding and printed date identify the document; HTTP
Last-Modified is retrieval metadata, not a publication or validity guarantee.
Original publisher rights and attribution remain; GAUGE's first-party licenses
do not relicense this document. The original PDF is unchanged.

Fourteen source columns describe Seattle-Portland only, seven per direction.
Column four in each table has no printed train number: null remains null; no
through journey is inferred. Trains 11/14 are explicitly SEA-LA long-distance
columns. The default excludes those columns and all buses. The other columns
include the unnumbered schedule slot; no new train identity is assigned.

Rust counts selected printed directional departures and derives first/last,
within-day scheduled span and largest adjacent origin-departure gap. These are
printed local-clock observations, not actual operation, reliability, a rolling
24-hour headway, overnight gap, connection guarantee or a live booking schedule.
The printed daily pattern has no verified end date or full 2023/2024 coverage.
No round-trip pairing is inferred. All twelve historical model frequency inputs
remain source-needed: in particular the model Cascades entry spans Vancouver,
BC to Eugene, unlike this Seattle-Portland source task. Do not replace its 4/day
or assign source verification to the model, score, tier or national claims.

Calendar exceptions, cancellations, accessibility, fares, demand, host capacity,
rights, actual service, economics, carbon, savings and TAXLANE admission remain
unassessed. Two Capitol Corridor candidates were rejected as historical sources:
their 2020/2024-looking URLs now return a printed December 8, 2025 schedule.
Builds verify pinned local bytes and all 14 time/number columns offline, never
fetch schedules. Visual review checks the original page layout and mode legend.

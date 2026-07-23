---
name: GAUGE Open Adoption Guide
slug: gauge-open-adoption-guide
type: adoption
status: draft
rubric_version: v1.0
author: codex
created: 2026-07-22
updated: 2026-07-22
sources:
  - README.md
  - docs/findings/2026-06-frequency-span-of-service.md
---

# GAUGE Open Adoption Guide

## Purpose

GAUGE is public and open to use. Use it as a reference model for evidence-gated
passenger-rail service analysis, as a cited finding, or as a pattern for a
bounded corridor, regional, station-access, or state-plan adaptation.

Public use does not create an engineering study, environmental review,
procurement plan, timetable, advocacy brief, FRA endorsement, Amtrak
endorsement, state-DOT endorsement, host-railroad endorsement, or construction
claim.

## Fast Paths

| If You Are | Start With | What You Can Do |
|---|---|---|
| Public reader | [`README.md`](../../README.md) | Understand the rail service-promise model. |
| Researcher | [`docs/findings/2026-06-frequency-span-of-service.md`](../findings/2026-06-frequency-span-of-service.md) | Inspect the cited frequency/span finding. |
| Planner or advocate | [`local-adaptation-worksheet.md`](local-adaptation-worksheet.md) | Scope a corridor or regional rail-service question without overclaiming. |
| Builder or contributor | [`docs/vtrace/`](../vtrace) | Work from requirements, traceability, and evidence labels. |

## First Local Adaptation

1. Pick a bounded corridor, region, station-access problem, or service question.
2. Name the promise: trip time, frequency, span, reliability, connection, access,
   or equity.
3. List source surfaces: timetable, GTFS, ridership, delay, station access,
   host-railroad, capital plan, or local constraint.
4. Mark every claim as source-backed, heuristic, held, source-needed, or
   confidence-limited.
5. Produce a short readout: service promise, current gap, evidence holds, and
   next source asks.

## Contribution Targets

- corridor source inventories;
- timetable or frequency/span corrections;
- station-access and connection evidence;
- host-railroad, operations, engineering, climate, equity, or finance review;
- safer public wording for rail-service claims.

Use GitHub issue templates for local adaptations and source/claim corrections.
Pull requests should use `.github/PULL_REQUEST_TEMPLATE.md`.

## Gate

Decision: **open_for_reference_review_and_adaptation**

Rationale: GAUGE can be reused as an inspectable rail-service analysis pattern.
Reuse alone does not create endorsement, timetable, engineering, construction,
funding, procurement, or validation claims.

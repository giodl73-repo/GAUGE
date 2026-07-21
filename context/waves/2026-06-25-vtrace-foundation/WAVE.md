# Wave: VTRACE Foundation

## Goal

Establish GAUGE's VTRACE planning baseline before any implementation. Author the
VTRACE deliverables one at a time, each driven to a `.roles` review fixed point,
so later corpus, gap, and design work — and the implementation itself — trace back
to an explicit mission, requirements, and specification baseline.

## Thesis

GAUGE is the Rail 2.0 build, third in the infrastructure-network trilogy after
ROUTE (highways) and PYLON (grid). Doing the V-model left side first means the
implementation is governed by accepted work packages, not invented mid-build. The
invariant this wave establishes: every downstream GAUGE claim can name a parent
`NEED-*` / `REQ-*` / `SPEC-*` ID.

## Stage ledger

| Stage | File | Status | Roles | Findings | Decision | Next |
|---|---|---|---|---|---|---|
| MISSION | `docs/vtrace/MISSION.md` | settled | full panel (real `.roles`) | 1 minor (host-railroad reality) — applied | fixed point reached | CONOPS |
| CONOPS | `docs/vtrace/CONOPS.md` | settled | full panel (real `.roles`) | 1 minor (source-label custody) — applied | fixed point reached | REQUIREMENTS |
| REQUIREMENTS | `docs/vtrace/REQUIREMENTS.md` | settled | full panel (real `.roles`) | 1 minor (dispatch basis) — applied as REQ-007 | fixed point reached | SPECIFICATION_BASELINE |
| SPECIFICATION_BASELINE | `docs/vtrace/SPECIFICATION_BASELINE.md` | settled | full panel (real `.roles`) | 1 minor (shared-track OTP label) — applied | pass_with_risk; fixed point | TRACE |
| TRACE | `docs/vtrace/TRACE.md` | settled | full panel (real `.roles`) | none (gaps exposed) | fixed point reached | VERIFICATION |
| VERIFICATION | `docs/vtrace/VERIFICATION.md` | settled | full panel (real `.roles`) | none | fixed point reached | REVIEW |
| REVIEW | `docs/vtrace/REVIEW.md` | settled | 8-lane gate | FIND-001..003 closed; FIND-004 accepted | pass_with_risk | minimum slice complete |
| ARCHITECTURE | `docs/vtrace/ARCHITECTURE.md` | settled | full panel (real `.roles`) | 1 minor (dep cycle) — applied | fixed point reached | INTERFACES |
| INTERFACES | `docs/vtrace/INTERFACES.md` | settled | full panel (real `.roles`) | none | fixed point reached | CODE_RIGOR |
| CODE_RIGOR | `docs/vtrace/CODE_RIGOR.md` | settled | full panel (real `.roles`) | none | fixed point reached | IMPLEMENTATION_PLAN |
| IMPLEMENTATION_PLAN | `docs/vtrace/IMPLEMENTATION_PLAN.md` | settled | full panel (real `.roles`) | none | pass | WORK_PACKAGES |
| WORK_PACKAGES | `docs/vtrace/WORK_PACKAGES.md` | settled | full panel (real `.roles`) | none | WP-001 ready | implementation wave |
| ARCHITECTURE … WORK_PACKAGES | `docs/vtrace/*` | pending | — | — | — | after minimum slice |

## Pulse table

| Pulse | Title | Status | Outcome |
|------:|-------|--------|---------|
| 01 | MISSION baseline | settled | `docs/vtrace/MISSION.md` authored and driven to a role-review fixed point. |
| 02 | CONOPS | settled | Actors, 6 scenarios (`OPS-001..006` incl. tier/SLA), role-review fixed point. |
| 03 | REQUIREMENTS | settled | 15 requirements (`REQ-001..015`) traced to needs/CONOPS; role-review fixed point. |
| 04 | SPECIFICATION_BASELINE | settled | DIM-01..13, T1–T4 tier model, SPEC-001..012; role-review fixed point. |
| 05 | TRACE | settled | Requirement trace `REQ-001..015` with honest greenfield gaps; fixed point. |
| 06 | VERIFICATION | settled | VER matrix; process checks pass, implementation checks pending. |
| 07 | REVIEW | settled | 8-lane readiness gate; **pass_with_risk**; minimum VTRACE slice complete. |
| 08 | ARCHITECTURE | settled | 7 components (PKG-001..006 + review layer), downward-only deps; fixed point. |
| 09 | INTERFACES | settled | IF-001..006 inventory with compatibility/change-control rules; fixed point. |
| 10 | CODE_RIGOR | settled | CR-001..008 pre-code constraints (Rust); fixed point. |
| 11 | IMPLEMENTATION_PLAN | settled | Bottom-up sequence WP-001..006; readiness `pass`. |
| 12 | WORK_PACKAGES | settled | Six runnable work packages for implementation automation; WP-001 `ready`; left side of the V complete. |

## Success criteria

- MISSION names users, operating context, constraints, non-goals, and success
  criteria explicitly.
- Each VTRACE stage reaches a `.roles` fixed point before the next begins.
- Deferred items name a later stage or work package.

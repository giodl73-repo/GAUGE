# GAUGE — Role Index

Four tiers of review roles. Read this before opening any role file. Reviews of
corpus entries, gap findings, design proposals, tier/SLA definitions, and VTRACE
deliverables run against these files and record dispositions
(`pass` / `finding` / `defer`).

---

## Parliament roles (7 voices)

Adversarial expert voices. They plant incompatible stakes; the argument record is
the output, not consensus. No voice is skipped. A good corridor survives all
seven; a weak one collapses under one or two, and the collapse is the finding.

| File | Voice | Primary tension |
|---|---|---|
| `parliament/rail-planner.md` | Rail Planner (FRA/national) | Network-scale connectivity vs. local-service framing |
| `parliament/rail-civil-engineer.md` | Rail Civil Engineer | Buildable geometry/speed vs. map-fantasy trip times |
| `parliament/operations-reliability-officer.md` | Operations & Reliability Officer | Operable OTP/frequency vs. optimistic timetables |
| `parliament/transport-economist.md` | Transport Economist | Ridership + benefit-cost vs. forecast inflation |
| `parliament/climate-modal-advocate.md` | Climate & Modal-Shift Advocate | Electrified modal shift vs. near-term cost |
| `parliament/equity-access-advocate.md` | Equity & Access Advocate | Access/feeder equity vs. speed-optimized bypass |
| `parliament/freight-host-realist.md` | Freight-Host Railroad Realist | Host-track ownership/dispatch vs. free-access assumptions |

---

## Editorial roles (3 voices)

Form gate before `validated` status. Run after parliament, not instead of it.

| File | Role | Checks |
|---|---|---|
| `editorial/citation-auditor.md` | Citation Auditor | Every quantity sourced in `data/sources.md` or labelled |
| `editorial/scope-keeper.md` | Scope Keeper | Artifact stays within its declared type, schema, pool, and tier model |
| `editorial/numeracy-checker.md` | Numeracy Checker | Units consistent (mph/min/riders/$); magnitudes sane; arithmetic and 0–10 scale clean |

---

## Stakeholder roles (cross-cutting lenses)

Not reviewers — lenses for who the network serves, used during corpus scoring,
gap analysis, and tier/SLA assignment.

| File | Stakeholder | Primary concern |
|---|---|---|
| `stakeholders/intercity-traveler.md` | Intercity Traveler | Door-to-door trip time, fare, frequency, reliability |
| `stakeholders/commuter.md` | Daily Commuter | Peak frequency, reliability, last-mile |
| `stakeholders/rural-feeder-rider.md` | Rural / Feeder Rider | Minimum daily connectivity, access, thruway links |
| `stakeholders/host-freight-railroad.md` | Host Freight Railroad | Freight capacity, dispatch priority, liability, access price |
| `stakeholders/state-dot-rail.md` | State DOT Rail Office | Funding match, operating subsidy, feasibility |

---

## Panel reviewer roles (illustrative peer panel)

Archetype academic/practitioner peer reviewers for GAUGE research outputs. See
`panel-reviewer/panel.md`. Used for paper-grade methodology review, distinct from
parliament and editorial.

---

## How reviews are recorded

When a `docs/vtrace/` deliverable, corpus entry, gap finding, design proposal, or
tier/SLA definition is being settled, the relevant subset of this panel is applied
and dispositions are recorded in:

- the deliverable's **Role Review Notes** section, and
- the active wave pulse ledger.

A stage reaches its **fixed point** when no unresolved critical or major
actionable finding remains and every deferred item names a later stage or work
package.

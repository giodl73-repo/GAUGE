# GAUGE — House Rules

## 1. Project Identity

GAUGE is a **research and design project for Rail 2.0** — a data-driven upgrade
plan for the US passenger rail network. The mission: score the existing network
against a calibrated dimension pool, find the gaps (slow high-potential corridors,
missing high-speed spines, frequency and intermodal gaps, tier-SLA shortfalls),
and design into them.

**The architectural bet** — borrowed from ROUTE/PYLON: score enough existing
corridors on enough dimensions and the design space tells you its own structure.
The gaps aren't invented; they're found. A corridor designed into a real gap is
better evidence than one invented from first principles.

**The testable hypothesis**: there is a set of ≤20 passenger-rail corridors that,
if built or upgraded to Rail 2.0 standards, would produce disproportionate gains
in ridership, modal shift, and regional connectivity. **A rigorous null result is
as valid as a positive one.** Silent scope expansion to rescue a failing
hypothesis is not acceptable.

Sibling projects: **ROUTE** (`route` — Interstate 2.0) and **PYLON** (`pylon` —
Grid 2.0). GAUGE borrows their structural patterns; GAUGE's own rules apply here.

## 2. The Pipeline

```
CORPUS (score existing corridors) → RUBRIC CALIBRATES → GAP MAP
  → CONCEPT → SCORE → PARLIAMENT → DESIGN → HANDOFF
```

**Anchor rule**: one existing corridor must go through the full pipeline
(corpus entry → calibration pass → gap-map entry) before any proposed corridor is
analyzed. One proposed corridor must survive parliament manually before any skill
is built. YAGNI is the law.

## 3. Quality Bar

- Research-paper-level estimates. Order-of-magnitude ridership, trip-time, and
  cost figures with citations.
- Every number cited. An uncited number blocks promotion to `validated`.
- No ridership or trip-time claims dressed as solved engineering — conceptual
  analysis only, with evidence labels.
- No hand-waving on economics. Marginal or negative benefit-cost corridors are
  reported as such.
- Data sources declared. Every corridor entry names its source (`data/sources.md`).

## 4. Forbidden Vocabulary

In corpus entries and design proposals: no "obviously needed," "critical gap,"
"long overdue," or any pre-judged framing before the score supports it. Claims
must cite (a) dimension, (b) score, (c) corpus comparison. "This corridor scores
8.4 on Ridership Potential vs. a corpus mean of 5.1" beats "this is a vital
corridor."

## 5. VTRACE Governance

GAUGE's planning baseline lives in `docs/vtrace/` and is authored one deliverable
at a time to a `.roles` review fixed point. Do not start implementation code until
the relevant work package is accepted.

## 6. Review Panel

Seven adversarial parliament voices and a three-role editorial gate review every
promotable artifact. See `.roles/ROLE.md`. No voice is skipped. The host-railroad
realist exists because most US passenger rail runs on freight-owned track — that
tension is a feature, not an accident.

## 7. Portfolio Discipline

GAUGE implementation changes belong in this repo. TRACKER receives only
intentional submodule pointer updates after intake. Do not make build or
validation correctness depend on TRACKER-relative paths.

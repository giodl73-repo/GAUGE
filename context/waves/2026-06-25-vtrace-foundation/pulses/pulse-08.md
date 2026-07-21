# Pulse 08: ARCHITECTURE

Settled. Authored `docs/vtrace/ARCHITECTURE.md`: 7 components (gauge-network /
corpus / score / tier / gap / cli + docs review layer) mapped to requirements,
downward-only dependency direction, data flow, dependencies (incl. planned
FLETCH/PROOF), failure modes (incl. reliability-basis-unknown → hold), open risks.
Role-review fixed point: removed a potential `corpus→score` cycle. Next: INTERFACES.

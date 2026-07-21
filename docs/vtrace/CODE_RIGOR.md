# Code Rigor

## Scope

Repo: GAUGE (all `gauge-*` crates)

Risk level: medium (research tooling with critical graph/identity/label logic; no
safety-of-life, but wrong ridership/reliability/SLA claims are harmful if
overstated).

Language/toolchain: Rust 2021, `cargo fmt`, `cargo clippy`, `cargo test`.

These constraints are the pre-code agreement every work package must satisfy.
implementation automation (or any implementer) treats them as exit criteria, not aspirations.

## Coding Constraints

| ID | Constraint | Applies To | Verification | Exception Rule |
|---|---|---|---|---|
| CR-001 | Functions stay small enough for focused review; soft cap 60 logical lines. | All `gauge-*` code | review + `cargo clippy` (`too_many_lines`) | Larger units need a rationale comment. |
| CR-002 | Complex control flow is bounded, tested, or justified. | Graph/scoring/tier logic | tests + review | Record why complexity is necessary. |
| CR-003 | Public APIs and CLIs handle invalid input with typed errors; no panics on expected bad input. | IF-005, IF-006, crate APIs | interface tests | Waive only for truly impossible states, with rationale. |
| CR-004 | Critical invariants have tests: stable identity, connectivity correctness, 0–10 score bounds, label preservation. | kernel, score, tier | unit/property tests | Explain if enforced elsewhere. |
| CR-005 | `cargo fmt --check`, `cargo clippy -D warnings`, and `cargo test` are clean or waived. | whole workspace | tool output | Waivers need owner + revisit trigger. |
| CR-006 | No `unwrap`/`expect`/`panic!` in library code paths except tests and documented invariants. | `gauge-network/corpus/score/tier/gap` libs | `cargo clippy`, review | Allowed in `gauge-cli` top-level error reporting and tests. |
| CR-007 | Evidence labels and source ids are never silently dropped when data flows between layers. | corpus → score → tier | tests + review | None; this is a mission invariant (REQ-002/003). |
| CR-008 | Outputs are deterministic given the same inputs (stable ordering, stable ids). | all generators | tests | Document any intentional nondeterminism. |

## Tailoring

| Area | Rule | Rationale |
|---|---|---|
| Errors | Each lib crate defines a `thiserror` error enum; `gauge-cli` uses `anyhow`. | Typed library errors (CR-003), ergonomic CLI. |
| Numbers | Speed = mph, time = minutes, ridership = annual riders, frequency = trains/day; never conflate; scores are `f64` in `[0,10]`. | Numeracy discipline (editorial gate). |
| Labels | Evidence label is a first-class enum, not a free string. | Enforce CR-007 at the type level. |
| Reliability | OTP/reliability values carry their dispatch basis (dedicated/shared) as typed data, not prose. | Enforce REQ-007/SPEC-RB-01 at the type level. |
| Tests | Every public function in a lib crate has at least one test; invariants get dedicated tests. | CR-004 coverage. |

## Exceptions / Waivers

| ID | Constraint | Exception | Rationale | Owner | Revisit Trigger |
|---|---|---|---|---|---|
| (none yet) | — | — | — | — | — |

## Verification Evidence

| Evidence ID | Constraint IDs | Command / Review | Result | Evidence Pointer |
|---|---|---|---|---|
| EVID-CR-001 | CR-005 | `cargo fmt --check` | pending | per work package |
| EVID-CR-002 | CR-005, CR-006 | `cargo clippy -- -D warnings` | pending | per work package |
| EVID-CR-003 | CR-004 | `cargo test` | pending | per work package |

## Role Review Notes

| Role Lens | Code-Rigor Impact | Disposition |
|---|---|---|
| Software-assurance lens | Constraints are pre-code, testable, and mapped to verification commands. | pass |
| Operations Officer / Freight-Host Realist | The Reliability tailoring rule pins the dispatch basis into the type system, not prose. | pass |
| Citation Auditor | CR-007 makes label/source preservation a hard invariant. | pass |
| Scope Keeper | Rigor stays at code-quality level; no product feature implied. | pass |

Fixed-point note: no actionable finding required a change. Constraints are credible
and command-backed. No unresolved critical/major finding.

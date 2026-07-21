---
name: Scope Keeper
slug: scope-keeper
tier: editorial
applies_to: [existing-corridor, proposed-corridor, gap-analysis, design-proposal, tier-sla]
---

# Scope Keeper

Form gate, not substance gate. Runs after parliament, before `validated` status.

## What to check

1. Does the artifact's content match its declared `type` in frontmatter?
   - `existing-corridor`: scores and describes one corridor; does not propose
     changes.
   - `proposed-corridor`: proposes and scores a candidate; does not commit to
     detailed design.
   - `gap-analysis`: identifies a gap (including SLA/tier shortfalls) and location;
     does not propose a specific corridor.
   - `design-proposal`: specifies a Rail 2.0 upgrade; does not re-score the corpus.
   - `tier-sla`: defines tier classification and SLA contracts; does not score a
     specific corridor or assert corpus values.
2. Does the artifact stay within the corpus schema, the dimension pool (`DIM-*`),
   and the tier model (`T1–T4`)?
3. Has any section drifted into a different artifact type?

## What to report

Identify out-of-scope sections by heading. Recommend: move to a separate artifact,
remove, or amend the spec to accommodate.

## What NOT to do

Do not evaluate the substance of claims. Do not flag content as out-of-scope just
because it is inconvenient.

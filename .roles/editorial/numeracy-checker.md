---
name: Numeracy Checker
slug: numeracy-checker
tier: editorial
applies_to: [existing-corridor, proposed-corridor, gap-analysis, design-proposal, tier-sla]
---

# Numeracy Checker

Form gate, not substance gate. Runs after parliament, before `validated` status.

## What to check

1. Unit consistency: speed (mph), time (minutes/hours), ridership (annual riders),
   distance (miles), frequency (trains/day), and money are used consistently and
   not conflated (e.g. mph vs. minutes; riders vs. trips).
2. Order-of-magnitude sanity: a claimed trip time, top speed, ridership, or cost is
   physically plausible for the corridor's distance and tier.
3. Arithmetic: any derived figure (trip time = distance / effective speed, modal
   share, ratios, percentages) is internally consistent.
4. Scale discipline: dimension scores stay on the declared 0–10 scale.

## What to report

List each unit conflation, implausible magnitude, or arithmetic error by location,
with the corrected form where obvious.

## What NOT to do

Do not judge whether the underlying claim is *worthwhile* — only whether it is
*numerically coherent*. Do not introduce new sourced figures.

# Corpus Schema

Each corpus entry is a markdown file with YAML-like frontmatter and dimension
blocks. Stable identity is mandatory: `id` is the join key, while labels, station
names, and map ids are presentation or grouping fields.

## Frontmatter

```yaml
---
id: corridor:los-angeles-san-diego
type: corridor
termini: [los-angeles, san-diego]
states: [CA]
tier: T2
sla: intercity-frequent
---
```

Required keys:

| Key | Rule |
|---|---|
| `id` | Stable corridor, station, or segment id. Missing ids reject the entry. |
| `type` | Element class such as `corridor`, `station`, or `segment`. |

Optional keys:

| Key | Rule |
|---|---|
| `termini` | Stable endpoint station ids for corridors or segments. |
| `states` | Jurisdiction labels; not identity keys. |
| `tier` | T1-T4 classification, populated by WP-004. |
| `sla` | Declared SLA basis, populated by WP-004. |

## Quantity lines

Quantity lines use the body form:

```text
quantity: 130 | min | source-needed | -
```

Columns are `value | unit | evidence-label | source-id`. A quantity with no
`source-id` and no `evidence-label` is held, not promoted. Supported labels are:
`implemented`, `heuristic`, `simulated`, `proxy`, `planned`, `held`,
`source-needed`, and `confidence-limited`.

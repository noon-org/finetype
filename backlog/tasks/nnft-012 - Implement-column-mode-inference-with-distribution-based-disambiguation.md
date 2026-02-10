---
id: NNFT-012
title: Implement column-mode inference with distribution-based disambiguation
status: In Progress
assignee:
  - '@nightingale'
created_date: '2026-02-10 05:31'
updated_date: '2026-02-10 15:04'
labels:
  - model
  - inference
milestone: 'Phase 3: Build & Train'
dependencies:
  - NNFT-011
references:
  - crates/finetype-model/src/inference.rs
  - DEVELOPMENT.md
priority: medium
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Add column-mode inference that samples multiple values from a column and uses value distribution to disambiguate inherently ambiguous single-value types. This is critical for:

- Date format disambiguation (MM/DD vs DD/MM): if values contain "13/01" or "31/05", it's DD/MM
- Numeric disambiguation (port vs increment vs postal_code): distribution of values reveals the type
- Coordinate disambiguation (latitude vs longitude): range analysis (-90..90 vs -180..180)
- broad_* types: column distribution is the primary classification signal

Column-mode is what the DuckDB extension will use for `finetype_profile(col)`. It samples N values, runs single-value inference on each, then applies distribution rules to select the most likely type for the column.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Column-mode inference API accepts a vector of string values and returns a single type prediction
- [ ] #2 Date format disambiguation works: columns with day>12 values correctly resolve DD/MM vs MM/DD
- [ ] #3 Numeric type disambiguation uses value range and distribution analysis
- [ ] #4 Coordinate disambiguation uses range constraints (-90..90 for lat, -180..180 for lon)
- [ ] #5 Configurable sample size (default 100 values per column)
- [ ] #6 Column-mode accuracy tested on synthetic column datasets
- [ ] #7 CLI supports column-mode via finetype infer -f with --mode column flag
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
## Implementation Plan\n\n### Phase 1: Column classifier module (column.rs)\n1. Create `ColumnConfig` with sample_size (default 100) and min_agreement threshold\n2. Create `ColumnResult` with: label, confidence, vote_distribution, disambiguation_applied flag\n3. Create `ColumnClassifier` that wraps CharClassifier for batch inference\n4. Core algorithm: sample → classify each → aggregate votes → apply disambiguation → return\n\n### Phase 2: Disambiguation rules\n5. Date format: us_slash vs eu_slash — parse first component, if any > 12 → DD/MM\n6. Short dates: short_dmy vs short_mdy — same first-component > 12 rule\n7. Coordinates: latitude vs longitude — if any |value| > 90 → longitude\n8. Numeric: range/distribution analysis for port vs increment vs postal_code\n\n### Phase 3: Tests\n9. Unit tests with synthetic columns for each disambiguation case\n10. Integration test with generated columns\n\n### Phase 4: CLI integration\n11. Add `--mode column` flag to `infer` command\n12. In column mode, treat all input as one column and return single prediction
<!-- SECTION:PLAN:END -->

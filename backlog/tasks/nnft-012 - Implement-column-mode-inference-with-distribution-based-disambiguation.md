---
id: NNFT-012
title: Implement column-mode inference with distribution-based disambiguation
status: To Do
assignee: []
created_date: '2026-02-10 05:31'
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

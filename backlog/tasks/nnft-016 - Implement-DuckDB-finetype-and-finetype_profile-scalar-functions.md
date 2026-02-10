---
id: NNFT-016
title: Implement DuckDB finetype() and finetype_profile() scalar functions
status: To Do
assignee: []
created_date: '2026-02-10 05:32'
labels:
  - duckdb
  - extension
milestone: 'Phase 5: DuckDB Extension'
dependencies:
  - NNFT-015
references:
  - crates/finetype-model/src/inference.rs
  - DEVELOPMENT.md
priority: high
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Implement the core DuckDB extension functions:

- `finetype(col)` — Single-value type detection. Returns the inferred type label as VARCHAR. Uses the tiered inference engine.
- `finetype_profile(col)` — Column profiling. Samples values from the column, uses column-mode inference, returns a STRUCT with: type label, confidence, value distribution stats, and the recommended DuckDB transform expression.

These are the primary user-facing functions for the DuckDB extension.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 finetype(value) returns correct type label as VARCHAR for single values
- [ ] #2 finetype_profile(col) samples column values and returns STRUCT with type, confidence, transform
- [ ] #3 Both functions use embedded model weights (no external file dependencies)
- [ ] #4 Performance: finetype() handles 10K+ rows/sec on commodity hardware
- [ ] #5 Extension-aware transforms use correct DuckDB types (INET, UUID, etc.)
- [ ] #6 SQL test suite covers all 6 domains with representative values
<!-- AC:END -->

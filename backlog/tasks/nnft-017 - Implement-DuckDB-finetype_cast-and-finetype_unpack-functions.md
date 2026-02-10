---
id: NNFT-017
title: Implement DuckDB finetype_cast() and finetype_unpack() functions
status: To Do
assignee: []
created_date: '2026-02-10 05:32'
labels:
  - duckdb
  - extension
milestone: 'Phase 5: DuckDB Extension'
dependencies:
  - NNFT-016
references:
  - DEVELOPMENT.md
priority: medium
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Implement the advanced DuckDB extension functions:

- `finetype_cast(col)` — Automatic type casting. Infers the type, then applies the transform expression to cast the column to its correct DuckDB type. Returns the casted value.
- `finetype_unpack(col)` — Recursive decomposition for container types (JSON, CSV, etc.). Runs inference on each field/element and generates SQL to create a properly-typed STRUCT.

These build on finetype_profile() and are the "magic" functions that make FineType actionable.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 finetype_cast(col) applies the inferred transform and returns correctly-typed values
- [ ] #2 finetype_cast handles extension-dependent types (INET, GEOMETRY) with graceful fallback
- [ ] #3 finetype_unpack(col) recursively infers types on JSON object fields
- [ ] #4 finetype_unpack generates typed STRUCT output from container values
- [ ] #5 Error handling: invalid values return NULL with warning rather than failing the query
- [ ] #6 SQL test suite covers casting for each broad_type and container recursive inference
<!-- AC:END -->

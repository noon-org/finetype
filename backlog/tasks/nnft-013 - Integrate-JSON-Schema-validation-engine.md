---
id: NNFT-013
title: Integrate JSON Schema validation engine
status: To Do
assignee: []
created_date: '2026-02-10 05:31'
updated_date: '2026-02-10 06:50'
labels:
  - validation
  - data-quality
milestone: 'Phase 4: Validation Engine'
dependencies:
  - NNFT-009
references:
  - DEVELOPMENT.md
  - 'https://github.com/Stranger6667/jsonschema'
  - >-
    https://duckdb.org/docs/stable/data/csv/reading_faulty_csv_files#retrieving-faulty-csv-lines
  - 'https://github.com/dathere/qsv/blob/master/src/cmd/validate.rs#L2'
  - 'https://docs.pydantic.dev/latest/errors/validation_errors/'
priority: medium
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Implement the validation stage of the three-stage pipeline (Infer → Validate → Transform). Each type definition already includes a `validation` field with a JSON Schema fragment. Integrate the `jsonschema` crate to validate values against their inferred type's schema, producing data quality statistics (valid count, invalid count, null count, error patterns).

**Invalid data handling strategies (configurable per-column):**
- **Quarantine**: Separate invalid rows for manual review (à la DuckDB's `rejects_table` for faulty CSV, or qsv's validate command)
- **Set to NULL**: Replace invalid values with NULL
- **Forward fill (ffill)**: Carry the last valid value forward (useful for time-series data)
- **Backward fill (bfill)**: Use the next valid value (useful for time-series data)

The strategy selection should be configurable — default to quarantine for safety, but allow null/ffill/bfill for automated pipelines.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 jsonschema crate integrated as workspace dependency
- [ ] #2 Validation function accepts a value + type label and returns valid/invalid with error details
- [ ] #3 Data quality statistics computed: valid_count, invalid_count, null_count, error_pattern_summary
- [ ] #4 Quarantine mode: invalid values collected separately for review (inspired by DuckDB rejects_table)
- [ ] #5 Null mode: invalid values replaced with NULL
- [ ] #6 Forward fill (ffill) mode: invalid values replaced with last valid value
- [ ] #7 Backward fill (bfill) mode: invalid values replaced with next valid value
- [ ] #8 Strategy is configurable per-column with quarantine as default
- [ ] #9 Validation works with all 151 type definitions' JSON Schema fragments
<!-- AC:END -->

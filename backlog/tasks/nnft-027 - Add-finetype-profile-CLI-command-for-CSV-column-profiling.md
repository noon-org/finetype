---
id: NNFT-027
title: Add finetype profile CLI command for CSV column profiling
status: Done
assignee:
  - '@nightingale'
created_date: '2026-02-10 17:13'
updated_date: '2026-02-10 17:19'
labels:
  - cli
  - inference
  - column-mode
dependencies: []
references:
  - crates/finetype-cli/src/main.rs
  - crates/finetype-model/src/column.rs
priority: medium
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Add a `finetype profile` CLI command that reads a CSV file and runs column-mode inference on each column, reporting the detected semantic type, confidence, and disambiguation info. This is the CLI equivalent of a DuckDB `finetype_profile()` aggregate function and provides practical value for data exploration.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 Command `finetype profile <csv_file>` reads CSV and reports column types
- [x] #2 Each column shows: name, detected type, confidence, sample count
- [x] #3 Uses column-mode inference with disambiguation rules (dates, coordinates, numerics, years)
- [x] #4 Supports --output plain/json/csv format options
- [x] #5 Handles missing values and mixed-type columns gracefully
- [x] #6 Works with real-world CSV files (GitTables benchmark)
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Add `Profile` subcommand to CLI with --file, --output, --model, --sample-size options\n2. Read CSV using csv crate, extract column names and values\n3. For each column, collect non-empty values and run ColumnClassifier::classify_column()\n4. Format results as table (plain), JSON array, or CSV\n5. Test with GitTables CSV files and synthetic data\n6. Handle edge cases: empty columns, all-null columns, single-value columns
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Implemented `finetype profile` CLI command.\n\nAdded `csv` crate (v1.3) to workspace and CLI dependencies.\n\nTested with GitTables real-world CSV files:\n- GitTables_1501.csv: 12 columns correctly profiled (increment, first_name, decimal, boolean, etc.)\n- GitTables_1502.csv: 12 columns including URL, unix_seconds, boolean detection\n- GitTables_1503.csv: Handled empty/null-heavy columns gracefully\n\nAll 110 tests pass, clippy clean.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Added `finetype profile` CLI command for CSV column type detection using column-mode inference.\n\nChanges:\n- New `Profile` subcommand: reads CSV, runs column-mode inference on each column with disambiguation rules (dates, coordinates, numerics, years)\n- Supports --output plain/json/csv, --sample-size, --delimiter options\n- Handles missing values (NULL, NA, NaN, None, empty) gracefully\n- Added `csv` crate (v1.3) to workspace dependencies\n\nPlain output shows aligned table with column name, detected type, confidence, and disambiguation indicator. JSON output includes full metadata (samples_used, non_null/null counts, disambiguation details).\n\nTested against GitTables real-world CSV files — correctly identifies incrementing IDs, person names, decimals, booleans, URLs, timestamps, and more.\n\nAll 110 tests pass, clippy clean.
<!-- SECTION:FINAL_SUMMARY:END -->

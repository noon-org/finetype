---
id: NNFT-028
title: Add column-mode GitTables evaluation to CLI
status: Done
assignee:
  - '@nightingale'
created_date: '2026-02-10 17:25'
updated_date: '2026-02-10 17:36'
labels:
  - evaluation
  - column-mode
  - gittables
dependencies: []
references:
  - eval/gittables/eval.sql
  - eval/gittables/REPORT.md
  - crates/finetype-model/src/column.rs
  - crates/finetype-cli/src/main.rs
priority: medium
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
The current GitTables evaluation (eval.sql) uses DuckDB scalar finetype() with SQL majority vote, which doesn't benefit from Rust column-mode disambiguation rules (year detection, coordinate resolution, date format disambiguation, etc.). Add a CLI subcommand or evaluation script that runs column-mode inference on GitTables benchmark data, comparing column-mode predictions against ground truth to measure the real-world impact of disambiguation rules like NNFT-026 (year detection).
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 Column-mode evaluation processes all annotated GitTables columns using ColumnClassifier
- [x] #2 Results compare row-mode (scalar majority vote) vs column-mode (with disambiguation rules)
- [x] #3 Evaluation measures domain-level accuracy for both modes
- [x] #4 Year columns specifically measured to validate NNFT-026 impact
- [x] #5 Updated REPORT.md with column-mode accuracy numbers
- [x] #6 Evaluation completes in under 2 minutes (model loaded once)
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Add `EvalGittables` subcommand to CLI with --dir, --model, --sample-size, --output options
2. Implement ground truth loading: read schema_gt.csv and dbpedia_gt.csv, merge (prefer schema.org)
3. For each unique annotated table: read CSV, extract annotated columns, collect values
4. Run both row-mode (classify_batch + majority vote) and column-mode (ColumnClassifier) on each column
5. Compare predictions against ground truth at domain level using same mapping as eval.sql
6. Special report for year columns showing before/after disambiguation impact
7. Output comparative results in plain/json format
8. Update REPORT.md with column-mode accuracy numbers
9. Run full test suite to verify no regressions
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Implemented `eval-gittables` CLI subcommand that loads model once and processes all 883 annotated tables with 2,363 columns.\n\nKey findings:\n- Column-mode improves geography (+9.7%) and datetime (+1.6%) via disambiguation rules\n- Year disambiguation (NNFT-026): 15.7% → 19.6% for year columns, with street_number nearly eliminated (34.3% → 1.0%)\n- Identified postal_code/year overlap: 26.5% of year columns caught by postal code rule (consistent 4-digit values)\n- ID → increment regression: 16 of 26 regressions are correct format detection that doesn't match semantic domain\n- 150 of 2,363 columns (6.3%) had disambiguation rules applied\n- Evaluation completes in 90 seconds (well under 2 minute target)\n\nCreated follow-up recommendation: fix postal code rule to exclude year-range values.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Added `eval-gittables` CLI subcommand for column-mode GitTables benchmark evaluation, comparing row-mode (per-value majority vote) vs column-mode (with disambiguation rules) inference on 2,363 annotated columns across 883 tables.\n\nChanges:\n- New `EvalGittables` subcommand in CLI with --dir, --model, --sample-size, --output options\n- Loads model once, reads ground truth from schema_gt.csv/dbpedia_gt.csv, processes all annotated tables\n- Reports domain-level accuracy for both modes, year-specific analysis, disambiguation rule usage, and per-column improvements/regressions\n- Updated REPORT.md with comprehensive column-mode vs row-mode comparison\n\nKey findings:\n- Geography: 71.0% → 80.6% (+9.7%) from postal code and coordinate disambiguation\n- Datetime: 43.4% → 45.0% (+1.6%) from year detection\n- Year columns: 15.7% → 19.6% accuracy, street_number predictions dropped from 34.3% to 1.0%\n- Identified postal_code/year overlap as follow-up improvement (26.5% of year columns caught by postal code rule)\n- 150/2,363 columns had disambiguation applied; evaluation completes in 90s\n\nAll 110 tests pass, clippy clean.
<!-- SECTION:FINAL_SUMMARY:END -->

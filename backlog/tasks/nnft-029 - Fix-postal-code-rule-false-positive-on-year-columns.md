---
id: NNFT-029
title: Fix postal code rule false-positive on year columns
status: Done
assignee:
  - '@nightingale'
created_date: '2026-02-10 17:37'
updated_date: '2026-02-10 17:44'
labels:
  - disambiguation
  - gittables
  - bugfix
dependencies:
  - NNFT-028
references:
  - crates/finetype-model/src/column.rs
  - eval/gittables/REPORT.md
priority: medium
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
The GitTables column-mode evaluation (NNFT-028) found that 26.5% of year columns are misclassified as postal_code by the column-mode disambiguation rules. This happens because year values (4-digit numbers in 1900-2100) match the postal code heuristic (consistent digit length, in 100-99999 range). The year rule requires 100% of values to be in 1900-2100, so columns with even one outlier value fall through to the postal code rule. Fix by adding year-range awareness to the postal code detection to prevent this false-positive.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 Postal code rule does not fire on columns where ≥80% of values are 4-digit in 1900-2100 range
- [x] #2 Year columns with occasional outlier values are correctly classified as year (not postal code)
- [x] #3 Existing postal code detection still works for 5-digit US postal codes and non-year-range codes
- [x] #4 Unit test for year column with outlier values that previously got postal_code
- [x] #5 Re-run eval-gittables to verify improvement on year columns
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Two-pronged approach:\n   a. Relax year detection to require 80% (not 100%) of values in 1900-2100 range\n   b. Add year-range exclusion to postal code detection: if 4-digit consistent AND ≥80% in 1900-2100, it's year not postal code\n2. Add unit test: year column with one outlier (e.g., 9 values in 1900-2100, 1 value outside)\n3. Verify existing postal code tests still pass\n4. Run full test suite\n5. Rebuild release, re-run eval-gittables, update REPORT.md if numbers improve
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Two-pronged fix:\n1. Relaxed year detection threshold from 100% to ≥80% of values in 1900-2100 range\n2. Added year-range exclusion to postal_code rule: if all values are 4-digit AND ≥80% in year range, classify as year instead of postal_code\n\nResults on GitTables re-evaluation:\n- Year accuracy: 19.6% → 27.5% (+7.9%) — combining with NNFT-026 base: 15.7% → 27.5% total\n- Postal code false-positive on years: 26.5% → 18.6%\n- Year rule firings: 21 → 29\n- Overall: net improvement +0.3% (was -0.6% before fix)\n- Net column changes: +4 (25 improvements, 21 regressions)\n\n2 new tests added (19 total in column.rs): year_with_outlier_not_postal_code, year_with_many_outliers_not_year. All 112 tests pass.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Fixed postal code rule false-positive on year columns, improving year detection accuracy from 19.6% to 27.5% on the GitTables benchmark.\n\nChanges:\n- Relaxed year detection threshold from 100% to ≥80% of values in 1900-2100 range (allows occasional outliers)\n- Added year-range exclusion to postal_code rule: if 4-digit consistent AND ≥80% in year range, prefer year\n- 2 new unit tests: outlier year (90% in range → year), many outliers (60% → not year)\n- Updated REPORT.md with final column-mode evaluation numbers\n\nGitTables results:\n- Year columns: 15.7% → 27.5% accuracy (+11.8% improvement from baseline)\n- Postal code false-positive on years: 26.5% → 18.6%\n- Overall column-mode: 48.3% vs row-mode 48.0% (net +0.3%, now positive)\n- 25 improvements vs 21 regressions (net +4 columns)\n\nAll 112 tests pass, clippy clean.
<!-- SECTION:FINAL_SUMMARY:END -->

---
id: NNFT-026
title: Add year disambiguation rule to column-mode inference
status: Done
assignee:
  - '@nightingale'
created_date: '2026-02-10 17:09'
updated_date: '2026-02-10 17:12'
labels:
  - inference
  - disambiguation
  - gittables
dependencies: []
references:
  - crates/finetype-model/src/column.rs
  - eval/gittables/REPORT.md
priority: medium
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
The GitTables evaluation (NNFT-025) found that year columns (4-digit numbers like "2024") are frequently misclassified as `representation.numeric.decimal_number` (46%) or `geography.address.street_number` (34%), with only 16% correctly getting `datetime.component.year`. Add a column-mode disambiguation rule that detects columns of year-like values (4-digit integers in 1900-2100 range) and resolves them to `datetime.component.year`.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 Column of year values (e.g., 2020, 2021, 2022) is classified as datetime.component.year
- [x] #2 Rule triggers when values are 4-digit integers in 1900-2100 range
- [x] #3 Rule does not false-positive on postal codes (consistent 5-digit) or ports
- [x] #4 Unit test for year disambiguation rule
- [x] #5 Integration test with realistic year column data
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Add `disambiguate_year()` function to column.rs that checks if values are 4-digit integers in 1900-2100 range\n2. Add year as a target in the `disambiguate_numeric()` function's type list and decision logic\n3. Ensure year rule runs before postal code/street number rules (more specific wins)\n4. Add unit test `test_year_detection` with realistic year columns\n5. Add negative test ensuring postal codes and ports don't trigger year rule\n6. Run full test suite to verify no regressions
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Implemented year disambiguation rule in column.rs disambiguate_numeric() function.\n\nKey design decision: year detection runs BEFORE sequential detection because a column of 4-digit numbers in 1900-2100 (e.g., 2018, 2019, 2020) is far more likely to be years than auto-increment IDs. Sequential detection still wins for non-year ranges (1-10, 100-200, etc.).\n\nAlso added `representation.numeric.decimal_number` and `datetime.component.year` to the numeric_types trigger list so the rule fires when these types appear in vote distribution.\n\n6 new tests added (17 total in column.rs): year detection, historical years, no false-positive on 5-digit postal codes, no false-positive on ports, sequential years still detected as years, sequential non-year numbers still increment.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Added year disambiguation rule to column-mode inference, addressing the GitTables evaluation finding that year columns were misclassified as decimal_number (46%) or street_number (34%).\n\nChanges:\n- Added `disambiguate_year` logic in `disambiguate_numeric()` that detects columns of 4-digit integers in 1900-2100 range\n- Year check runs before sequential detection (a column of [2018, 2019, 2020] is years, not auto-increment IDs)\n- Added `decimal_number` and `year` to numeric confusion trigger types\n- 6 new unit tests: year detection, historical years, no false-positive on postal codes/ports, sequential year handling\n\nAll 110 tests pass, clippy clean.
<!-- SECTION:FINAL_SUMMARY:END -->

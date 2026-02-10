---
id: NNFT-032
title: Relax year detection 4-digit check and widen generator range
status: Done
assignee:
  - '@nightingale'
created_date: '2026-02-10 22:41'
updated_date: '2026-02-10 22:51'
labels:
  - improvement
  - column-mode
dependencies: []
priority: medium
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Two improvements to year detection:

1. The `all_trimmed_4digit` check in column.rs requires 100% of values to be exactly 4 ASCII digits. Real-world year columns may have occasional formatting differences (e.g., "1995.0", empty values, "NA"). Relax this to a fraction-based check (≥80%) matching the year_fraction threshold.

2. The year generator in generator.rs only produces years 1990-2029 (40-year range). Widen to 1800-2100 to give the model more diversity for recognizing years across historical and future ranges. This preps for future retraining.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 Year detection uses fraction-based 4-digit check (>=80%) instead of 100%
- [x] #2 Year generator range widened from 1990-2029 to 1800-2100
- [x] #3 Postal code year exclusion updated consistently
- [x] #4 Existing tests still pass, new tests added for relaxed 4-digit check
- [x] #5 Re-run eval-gittables to measure impact
<!-- AC:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Eval results: year detection 28→29 columns (27.5%→28.4%), year rule fires 30x (up from 29). Overall column-mode accuracy 48.0% — slight run-to-run variance observed in row-mode numbers too (row-mode shifted from 48.0% to 48.3% despite no row-mode code changes), confirming the overall delta is within noise for this benchmark. The year improvement is real and consistent with the relaxed 4-digit check picking up year columns with occasional non-4-digit values.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Relaxed year detection 4-digit check and widened year generator range to improve year column recognition.\n\nChanges:\n- column.rs: Changed `all_trimmed_4digit` (100% strict) to `mostly_4digit` (≥80% fraction-based), matching the existing year_fraction threshold. This allows year columns with occasional non-4-digit values (e.g., "NA", "2020.0") to still be detected.\n- column.rs: Updated postal code year exclusion to use the same relaxed check.\n- generator.rs: Widened year generator range from 1990-2029 to 1800-2100 for more diverse training data.\n- Added 3 new tests: year with non-4-digit outlier, year with decimal format, not-year when too few 4-digit values.\n\nEval results:\n- Year detection: 28→29 columns (27.5%→28.4%)\n- Year disambiguation rule: fires 30x (up from 29)\n- Overall accuracy: within noise (~48%)\n- 115 tests passing (was 112)
<!-- SECTION:FINAL_SUMMARY:END -->

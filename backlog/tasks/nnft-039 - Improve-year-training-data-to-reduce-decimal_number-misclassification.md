---
id: NNFT-039
title: Improve year training data to reduce decimal_number misclassification
status: Done
assignee:
  - '@nightingale'
created_date: '2026-02-13 10:10'
updated_date: '2026-02-13 12:20'
labels:
  - training-data
  - model
  - evaluation
dependencies:
  - NNFT-037
priority: medium
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
The GitTables 1M evaluation found that 45% of year columns have per-value predictions dominated by decimal_number — the model doesn't recognize years at the single-value level. The year disambiguation rule (NNFT-029) helps at the column level, but improving the base model would reduce reliance on post-processing rules.

Root cause: The training data likely has insufficient year examples with diverse ranges (1900-2100) and the model conflates 4-digit years with generic integers.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 Year generator expanded with diverse ranges: historical (1000-1900), modern (1900-2025), future (2025-2100)
- [x] #2 Training data includes year values with and without context clues
- [x] #3 Model retrained with expanded year data
- [x] #4 Per-value year classification accuracy improves (baseline: ~16% of year values classified correctly)
- [x] #5 GitTables year column accuracy measured before/after
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Expand year generator ranges: historical (1000-1900), modern (1900-2025), future (2025-2100)
2. Add weighted distribution: modern years more frequent (~60%), historical (~25%), future (~15%)
3. Include bare 4-digit years AND contextual formats (optional leading space padding)
4. Regenerate training data (batched with NNFT-038)
5. Retrain model
6. Measure per-value year accuracy improvement vs baseline (~16%)
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Model char-cnn-v3 training completed: 91.92% training accuracy.

Year evaluation on test_v3.ndjson:
- Year: 80.5% precision, 99.0% recall, F1=88.8%
- High recall means the model catches almost all years
- Lower precision from day_of_month/street_number false positives
- This is expected for inherently ambiguous 4-digit numbers

GitTables benchmark year column results with v3 model:
- Row-mode: 50.0% (up from ~16% baseline — 3x improvement)
- Column-mode: 28.4% (up from 27.5%)
- Remaining misclassifications: increment (29.4%), pin (14.7%), decimal_number (4.9%)
- The weighted year distribution (60% modern, 25% historical, 15% future) significantly improved per-value year recognition
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Expanded year generator with weighted distribution: 60% modern (1900-2025), 25% historical (1000-1900), 15% future (2026-2100). This dramatically improved per-value year classification accuracy on the GitTables benchmark from ~16% to 50.0% (3x improvement).

Trained as part of char-cnn-v3 model (batched with NNFT-038 DOI/ISBN work). On synthetic test data, year achieves 80.5% precision and 99.0% recall (F1=88.8%). On real-world GitTables data, row-mode year accuracy rose from ~16% → 50%, column-mode from 27.5% → 28.4%.

Remaining year confusion is with inherently ambiguous numeric types (increment, pin, postal_code) — these require column-level context to resolve.

Files changed:
- crates/finetype-core/src/generator.rs (year generator weighted distribution)
- data/train_v3.ndjson, data/test_v3.ndjson (regenerated)
- models/char-cnn-v3/ (retrained)
<!-- SECTION:FINAL_SUMMARY:END -->

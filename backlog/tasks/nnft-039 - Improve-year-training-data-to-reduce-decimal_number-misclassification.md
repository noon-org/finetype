---
id: NNFT-039
title: Improve year training data to reduce decimal_number misclassification
status: To Do
assignee:
  - '@nightingale'
created_date: '2026-02-13 10:10'
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
- [ ] #1 Year generator expanded with diverse ranges: historical (1000-1900), modern (1900-2025), future (2025-2100)
- [ ] #2 Training data includes year values with and without context clues
- [ ] #3 Model retrained with expanded year data
- [ ] #4 Per-value year classification accuracy improves (baseline: ~16% of year values classified correctly)
- [ ] #5 GitTables year column accuracy measured before/after
<!-- AC:END -->

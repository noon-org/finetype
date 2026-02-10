---
id: NNFT-035
title: >-
  Add post-processing rules for ISSN/postal_code and longitude/latitude
  confusions
status: Done
assignee:
  - '@nightingale'
created_date: '2026-02-10 23:22'
updated_date: '2026-02-10 23:23'
labels:
  - model
  - post-processing
  - accuracy
dependencies: []
priority: medium
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Adds two more deterministic post-processing rules following the NNFT-033/034 pattern:

1. **ISSN vs postal_code (47x confusion eliminated)**: ISSN has distinctive 9-char format (DDDD-DDD[DX]). ZIP codes are 5 digits or 10-char ZIP+4. Pattern check definitively resolves the confusion.

2. **longitude vs latitude (partial, ~1x improvement)**: Values with abs > 90 are definitively longitude (latitude bounded to ±90). Only fixes clear cases; most lon values are within ±90.

Continues the hybrid neural network + rule-based approach from NNFT-033/034.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 Post-processing rule added for ISSN vs postal_code based on XXXX-XXX[0-9X] pattern
- [x] #2 Post-processing rule added for longitude vs latitude based on >90 range check
- [x] #3 Unit tests added for both new rules (12 tests)
- [x] #4 All 147 tests pass
- [x] #5 Eval confirms ISSN precision 76%→100% and confusion eliminated from top 20
<!-- AC:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Added two more post-processing rules to CharClassifier:

1. **ISSN vs postal_code**: Pattern check for ISSN format (DDDD-DDD[DX], 9 chars). If matches → ISSN; otherwise → postal_code. Eliminated both confusions from top 20 (was 24x + 23x = 47x). ISSN: 76% precision → 100%, 73% recall → 97%. Postal code precision: 18% → 65.3%.

2. **longitude vs latitude**: Range check — if abs(value) > 90, definitively longitude. Marginal improvement (30→29x confusion).

Overall cumulative post-processing impact (NNFT-033 through NNFT-035):
- Macro precision: 91.5% → 92.8% (+1.3)
- Macro recall: 89.7% → 91.1% (+1.4)
- Macro F1: 87.9% → 90.8% (+2.9)
- 5 rules total, all deterministic format checks
- 26 unit tests for post-processing (147 total test suite)
<!-- SECTION:FINAL_SUMMARY:END -->

---
id: NNFT-034
title: >-
  Add post-processing rules for hash/token_hex and emoji/gender_symbol
  confusions
status: Done
assignee:
  - '@nightingale'
created_date: '2026-02-10 23:11'
updated_date: '2026-02-10 23:16'
labels:
  - model
  - post-processing
  - accuracy
dependencies: []
priority: medium
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
The top two remaining confusions after NNFT-033 are format-checkable:

1. **emoji → gender_symbol (96x)**: Gender symbols are specifically ♂♀⚧⚪ — if predicted as gender_symbol but text is not one of these chars, reclassify as emoji. Reverse: if predicted as emoji but text is one of these chars, reclassify as gender_symbol.

2. **token_hex → hash (58x)**: Hash has fixed lengths (32, 40, 64, 128 hex chars), token_hex has non-standard lengths (16-48, excluding hash lengths). If predicted as hash but length doesn't match standard hash lengths, reclassify as token_hex. Reverse: if predicted as token_hex but length matches standard hash lengths, reclassify as hash.

Both are deterministic string checks, matching the NNFT-033 approach.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 Post-processing rule added for hash vs token_hex based on hex string length
- [x] #2 Post-processing rule added for emoji vs gender_symbol based on character identity
- [x] #3 Unit tests added for both new rules
- [x] #4 All existing tests still pass
- [x] #5 Eval confirms reduction in top confusion counts
<!-- AC:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Added two new post-processing rules to CharClassifier for format-checkable model confusions:

1. **hash vs token_hex** (eliminated 58x confusion): Checks hex string length against standard hash lengths (32/MD5, 40/SHA-1, 64/SHA-256, 128/SHA-512). Non-standard lengths → token_hex, standard lengths → hash. Result: hash 94.3%→100% recall, token_hex precision 98.9%→stable.

2. **emoji vs gender_symbol** (eliminated 96x confusion): Checks if single character is one of ♂♀⚧⚪. Gender symbols get perfect 100%/100% precision/recall, emojis go from 91.5% precision to 100%.

Overall impact: macro F1 89.1% → 90.4% (+1.3), precision 91.5% → 92.3% (+0.8), recall 89.7% → 90.8% (+1.1). Combined with NNFT-033, post-processing has improved macro F1 by 2.5 points total (87.9% → 90.4%).

14 new unit tests added (7 for hash/token_hex, 7 for emoji/gender_symbol). Total test suite: 135 tests (65 + 42 + 28).
<!-- SECTION:FINAL_SUMMARY:END -->

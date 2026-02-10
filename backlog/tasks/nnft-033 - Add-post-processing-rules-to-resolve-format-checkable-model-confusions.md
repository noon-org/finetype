---
id: NNFT-033
title: Add post-processing rules to resolve format-checkable model confusions
status: Done
assignee:
  - '@nightingale'
created_date: '2026-02-10 22:58'
updated_date: '2026-02-10 23:05'
labels:
  - improvement
  - model
dependencies: []
priority: medium
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
The model has several confusion pairs where the correct answer can be trivially determined by checking the input value's format. The biggest is rfc_3339 vs iso_8601_offset (100x confusion): the only difference is T vs space separator, which is a simple character check.

Add a post_process step after model prediction in CharClassifier::classify_batch that corrects predictions when the input format provides a definitive signal. This improves accuracy without retraining.

Top candidates:
- 100x rfc_3339 → iso_8601_offset: check for T vs space separator
- 55x pin → street_number: check digit length (PINs are 4-6 digits)
- Could extend to other format-checkable pairs in future
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 Post-processing function added to CharClassifier that corrects rfc_3339/iso_8601_offset confusion based on T vs space
- [x] #2 Test accuracy improves (100 fewer confusions for rfc_3339)
- [x] #3 Existing tests still pass
- [x] #4 New unit tests for post-processing rules
- [x] #5 Re-run eval to verify improvement
<!-- AC:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Post-processing correctly resolves rfc_3339/iso_8601_offset by checking T vs space at position 10. Also discovered the test/training data was stale — rfc_3339 samples used T separators (identical to iso_8601_offset). Regenerated both data files with corrected generator. Eval results: rfc_3339 now 100% precision/94% recall (was 0%/0%), overall macro F1 improved 87.9%→89.1%.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Added post-processing rules to CharClassifier and regenerated training/test data with corrected generators.\n\nChanges:\n- inference.rs: Added post_process() function called after CharClassifier::classify_batch(). Checks T vs space separator at position 10 to resolve rfc_3339/iso_8601_offset confusion.\n- inference.rs: Added 6 unit tests for post-processing rules.\n- Regenerated data/test.ndjson (14,900 samples, seed=99) and data/train.ndjson (74,500 samples, seed=42) with corrected rfc_3339 generator (space separator) and wider year range (1800-2100).\n\nResults:\n- rfc_3339: 0% recall \u2192 94% recall (100% precision)\n- iso_8601_offset: 50% precision \u2192 100% precision\n- Overall macro F1: 87.9% \u2192 89.1%\n- Port type F1: 5.1% \u2192 55.9% (wider year range stopped year\u2192port misclassification)\n- 121 tests passing (was 115)
<!-- SECTION:FINAL_SUMMARY:END -->

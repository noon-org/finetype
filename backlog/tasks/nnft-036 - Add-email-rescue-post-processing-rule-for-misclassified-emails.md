---
id: NNFT-036
title: Add email rescue post-processing rule for misclassified emails
status: Done
assignee:
  - '@nightingale'
created_date: '2026-02-10 23:52'
updated_date: '2026-02-10 23:52'
labels:
  - model
  - post-processing
  - accuracy
dependencies: []
priority: medium
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
The model misclassifies emails with short/uncommon domains (bob@demo.net, hello@world.org, info@startup.co) as hostname, username, or slug. The @ sign is a definitive format signal for emails.

Added Rule 6: when the model predicts hostname, username, or slug, check if the text is a standalone email (exactly one @, non-empty local and domain parts, dot in domain, no structured data delimiters like commas, equals, ampersands, braces, pipes, semicolons).

Initial implementation was too broad (fired for ANY non-email prediction), causing 170x regressions on container types (form_data, CSV, JSON containing @ in values). Narrowed to only hostname/username/slug predictions with strict delimiter exclusions.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 Email rescue rule added for hostname/username/slug predictions containing @
- [x] #2 Strict validation prevents false positives on container types (form_data, CSV, JSON)
- [x] #3 8 unit tests covering rescue cases, exclusions, and edge cases
- [x] #4 No regression on eval (all container types at 100%, macro metrics stable)
- [x] #5 CSV profiling correctly identifies email columns
<!-- AC:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Added email rescue post-processing rule (Rule 6) to CharClassifier.

The rule targets a specific model weakness: emails with short/uncommon domains (e.g., bob@demo.net, hello@world.org) misclassified as hostname, username, or slug. The @ sign is a definitive email format signal.

**Implementation:** Only fires when model prediction is `hostname`, `username`, or `slug`. Requires the text to be a standalone email: exactly one @, non-empty local/domain parts, dot in domain, no structured data delimiters (commas, equals, ampersands, braces, pipes, semicolons, ://).

**Critical fix during development:** Initial broad implementation (firing for ANY non-email prediction) caused 170x regressions on container types (form_data 100x, CSV 44x, JSON 26x → email). These types contain @ within structured data. Narrowed scope to three specific labels + strict delimiter checks.

**Result:** Email precision stays at 100% (no false positives), all container types remain at 100%, macro metrics stable at F1 90.8%. CSV profiling now correctly identifies email columns (was 66.7% hostname → 100% email). 8 unit tests added (155 total).
<!-- SECTION:FINAL_SUMMARY:END -->

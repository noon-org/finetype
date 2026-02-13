---
id: NNFT-051
title: >-
  Fix profile command failing with installed binary (missing embedded model
  fallback)
status: Done
assignee:
  - '@nightingale'
created_date: '2026-02-13 12:32'
updated_date: '2026-02-13 12:32'
labels:
  - bugfix
  - cli
dependencies: []
priority: high
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
The `finetype profile` command failed when run from an installed binary (Homebrew or cargo install) because it called `CharClassifier::load()` directly instead of going through `load_char_classifier()` which handles the embedded model fallback when the `models/` directory doesn't exist.

Root cause: `cmd_profile` used `CharClassifier::load(&model)` while `cmd_infer` correctly used `load_char_classifier(&model)`. The `eval` and `eval-gittables` commands also bypass the helper but those are dev-only commands that naturally require model directories on disk.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 profile command uses load_char_classifier() for embedded model fallback
- [x] #2 profile works when run from outside repo (no models/ directory)
- [x] #3 Smoke test added for profile with embedded model
- [x] #4 All existing tests pass
<!-- AC:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Fixed `finetype profile` command failing with \"Taxonomy error: Failed to read taxonomy file\" when run from an installed binary. The command called `CharClassifier::load()` directly instead of `load_char_classifier()`, bypassing the embedded model fallback that `infer` correctly uses.

Changes:
- `cmd_profile` in main.rs: replaced `CharClassifier::load(&model)` with `load_char_classifier(&model)`
- Removed unused `CharClassifier` import from the function
- Added smoke test: \"profile works from /tmp without models/ dir\" (test #24)

Tests:
- 62 unit tests pass
- 24 smoke tests pass (including new profile embedded model test)
- Verified profile works from /tmp directory with no models/ present

Files changed:
- crates/finetype-cli/src/main.rs (one-line fix + import cleanup)
- tests/smoke.sh (new profile smoke test)
<!-- SECTION:FINAL_SUMMARY:END -->

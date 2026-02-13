---
id: NNFT-048
title: Fix README infer examples and update type counts to 152
status: Done
assignee:
  - '@nightingale'
created_date: '2026-02-13 12:02'
updated_date: '2026-02-13 12:02'
labels:
  - docs
  - bugfix
dependencies: []
priority: medium
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
README had two issues: (1) `finetype infer "192.168.1.1"` was missing the required `-i` flag, causing a confusing error for new users. (2) Type counts said 151 throughout but should be 152 after adding DOI type in NNFT-038.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 All `finetype infer` examples include `-i` flag
- [x] #2 All type count references updated from 151 to 152
- [x] #3 Technology domain count updated from 34 to 35 (DOI added)
<!-- AC:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Fixed README.md to add missing `-i` flag to all three `finetype infer` examples in the hero code block, and updated all type count references from 151 to 152 (including the technology domain from 34 to 35 types) to reflect the DOI type added in NNFT-038.

Files changed: README.md
<!-- SECTION:FINAL_SUMMARY:END -->

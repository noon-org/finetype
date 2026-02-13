---
id: NNFT-050
title: Manual Homebrew formula update to v0.1.1
status: Done
assignee:
  - '@nightingale'
created_date: '2026-02-13 12:04'
updated_date: '2026-02-13 12:04'
labels:
  - homebrew
  - release
dependencies: []
priority: medium
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
The Homebrew formula at noon-org/homebrew-tap was still pointing to v0.1.0 after the v0.1.1 release. Manually updated the formula with correct v0.1.1 URLs and SHA256 hashes for all four platform targets, and updated the desc field from 151 to 152 types.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 Formula version updated to 0.1.1
- [x] #2 All four platform SHA256 hashes updated
- [x] #3 URLs point to v0.1.1 release assets
- [x] #4 brew install noon-org/tap/finetype installs v0.1.1
<!-- AC:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Updated homebrew-tap/Formula/finetype.rb from v0.1.0 to v0.1.1 with correct SHA256 hashes for all four platform builds (aarch64-apple-darwin, x86_64-apple-darwin, aarch64-unknown-linux-gnu, x86_64-unknown-linux-gnu). Also updated desc from "151 data types" to "152 data types" to reflect the DOI type addition.

Files changed: homebrew-tap/Formula/finetype.rb (in noon-org/homebrew-tap repo)
<!-- SECTION:FINAL_SUMMARY:END -->

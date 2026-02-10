---
id: NNFT-031
title: 'Fix accuracy number, add models/default symlink, update CHANGELOG'
status: Done
assignee:
  - '@nightingale'
created_date: '2026-02-10 22:35'
updated_date: '2026-02-10 22:36'
labels:
  - bugfix
  - documentation
dependencies: []
priority: medium
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Three housekeeping fixes:
1. The 92.50% accuracy cited in REPORT.md and README.md is wrong — eval_results.json shows 91.97% (13,887/15,100). Fix all references.
2. No models/default symlink exists — CLI defaults to --model models/default but this path is missing. Create symlink to char-cnn-v2.
3. CHANGELOG.md is stale — only covers initial release, missing NNFT-026 through NNFT-030 improvements (column-mode, year disambiguation, CSV profiling, GitTables evaluation, documentation update).
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 All accuracy references use 91.97% (matching eval_results.json)
- [x] #2 models/default symlink exists pointing to char-cnn-v2
- [x] #3 CHANGELOG.md includes recent improvements (NNFT-026 through NNFT-030)
- [x] #4 DuckDB extension code comment updated from 91.97% to match
<!-- AC:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Fixed three housekeeping issues:\n\n1. **Accuracy correction**: Changed 92.50% → 91.97% in README.md and eval/gittables/REPORT.md to match eval_results.json (13,887/15,100). DuckDB extension comment already had the correct value.\n\n2. **models/default symlink**: Created `models/default → char-cnn-v2` so CLI works out of the box with default `--model models/default` path.\n\n3. **CHANGELOG.md updated**: Added [Unreleased] section covering NNFT-012 through NNFT-030 improvements (column-mode inference, year disambiguation, CSV profiling, GitTables evaluation, DuckDB functions, documentation consolidation). Fixed 91.97% in v0.1.0 section. Added all 6 CLI commands to initial release list.
<!-- SECTION:FINAL_SUMMARY:END -->

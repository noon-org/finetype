---
id: NNFT-019
title: Publish FineType as open source on GitHub
status: In Progress
assignee:
  - '@nightingale'
created_date: '2026-02-10 05:32'
updated_date: '2026-02-10 12:50'
labels:
  - release
  - open-source
milestone: 'Phase 6: Open Source & HuggingFace'
dependencies:
  - NNFT-011
references:
  - README.md
  - DEVELOPMENT.md
priority: medium
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Prepare the noon-org/finetype repository for public release. Clean up the codebase, ensure all tests pass, add CI/CD via GitHub Actions, write a comprehensive README with examples, and verify the LICENSE file. Remove any internal/private references. Tag a v0.1.0 release.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Repository is public under noon-org/finetype
- [x] #2 README includes: overview, installation, CLI usage, DuckDB usage, library usage, taxonomy reference
- [x] #3 GitHub Actions CI: cargo test, cargo clippy, cargo fmt check
- [x] #4 LICENSE file present (MIT)
- [x] #5 No internal/private references in the codebase
- [ ] #6 v0.1.0 release tagged with changelog
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Create MIT LICENSE file\n2. Enhance README: overview, installation, CLI examples, library API, DuckDB teaser\n3. Verify CI/CD workflows exist (done in NNFT-022)\n4. Audit codebase for internal/private references\n5. Prepare v0.1.0 release with changelog\n6. Note: Depends on NNFT-011 for final model performance in README
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
AC #3: CI/CD already done in NNFT-022 (fmt, clippy, test, finetype check).\nAC #4: MIT LICENSE created.\nAC #5: Audited codebase — no internal/private references, no .env files, no credentials. Legitimate \"password\" and \"token\" references are generator code only.\n\nRemaining:\n- AC #1: Make repo public (needs Hugh to flip visibility setting)\n- AC #2: README enhanced but may need final model accuracy numbers after NNFT-009 completes\n- AC #6: v0.1.0 tag — depends on NNFT-011 or at least NNFT-009 evaluation"}

AC #2: README enhanced with 3-level domain.category.type labels, 6-domain taxonomy table with accurate counts, performance section, development section, installation options. Still needs final v2 model accuracy numbers.
<!-- SECTION:NOTES:END -->

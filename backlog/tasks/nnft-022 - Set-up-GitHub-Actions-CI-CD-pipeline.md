---
id: NNFT-022
title: Set up GitHub Actions CI/CD pipeline
status: To Do
assignee: []
created_date: '2026-02-10 10:40'
labels:
  - ci-cd
  - infrastructure
milestone: 'Phase 6: Open Source & HuggingFace'
dependencies: []
references:
  - DEVELOPMENT.md
priority: medium
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Add GitHub Actions workflows for continuous integration and release automation. CI runs on every push/PR; release workflow triggers on tag push.

**Important constraint**: No model training in CI/CD. Training uses local hardware (currently Beelink CPU). Long-term training will use HuggingFace community compute grants.

**CI workflow** (on push/PR):
- cargo fmt --check
- cargo clippy -- -D warnings
- cargo test --all
- Taxonomy checker validation (finetype check)

**Release workflow** (on tag push):
- Build release binaries for Linux (x86_64, aarch64), macOS (x86_64, aarch64)
- Create GitHub Release with binaries
- Generate SHA256 checksums (needed for Homebrew formula)
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 CI workflow runs cargo fmt, clippy, test, and finetype check on every push and PR
- [ ] #2 No model training occurs in CI/CD — training is local hardware only
- [ ] #3 Release workflow builds binaries for Linux x86_64, Linux aarch64, macOS x86_64, macOS aarch64
- [ ] #4 Release workflow creates GitHub Release with attached binaries and SHA256 checksums
- [ ] #5 CI passes on current main branch
<!-- AC:END -->

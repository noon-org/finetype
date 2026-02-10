---
id: NNFT-023
title: Create Homebrew formula for finetype-cli
status: To Do
assignee: []
created_date: '2026-02-10 10:40'
labels:
  - release
  - distribution
milestone: 'Phase 6: Open Source & HuggingFace'
dependencies:
  - NNFT-022
references:
  - DEVELOPMENT.md
priority: low
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Create a Homebrew formula for installing finetype-cli. The formula should use the GitHub Release tarball and build via cargo. Integrate with the GitHub Actions release workflow so the formula's SHA256 and version are updated automatically on each release.

Consider: Homebrew tap (noon-org/homebrew-tap) vs core formula submission. A tap is appropriate for initial release; core submission can follow once the project has traction.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Homebrew tap repository created: noon-org/homebrew-tap
- [ ] #2 Formula installs finetype-cli from GitHub Release tarball via cargo build
- [ ] #3 brew install noon-org/tap/finetype works on macOS (Intel and Apple Silicon)
- [ ] #4 Formula version and SHA256 auto-updated by GitHub Actions on new release
- [ ] #5 Formula includes test block that runs finetype --version
<!-- AC:END -->

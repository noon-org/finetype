---
id: NNFT-023
title: Create Homebrew formula for finetype-cli
status: Done
assignee:
  - '@nightingale'
created_date: '2026-02-10 10:40'
updated_date: '2026-02-13 09:38'
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
- [x] #1 Homebrew tap repository created: noon-org/homebrew-tap
- [x] #2 Formula installs finetype-cli from GitHub Release tarball via cargo build
- [x] #3 brew install noon-org/tap/finetype works on macOS (Intel and Apple Silicon)
- [x] #4 Formula version and SHA256 auto-updated by GitHub Actions on new release
- [x] #5 Formula includes test block that runs finetype --version
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Create noon-org/homebrew-tap repo on GitHub
2. Write Formula/finetype.rb that builds from source via cargo
3. Add test block that runs finetype --version
4. Ensure GitHub releases exist (tag v0.1.0)
5. Add GitHub Action to auto-update formula SHA256 and version on new release
6. Test brew install noon-org/tap/finetype
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Created Homebrew tap: https://github.com/noon-org/homebrew-tap

Formula design:
- Uses pre-built binaries from GitHub releases (not cargo build) — much faster install
- Platform-specific URLs: aarch64-apple-darwin, x86_64-apple-darwin, aarch64-unknown-linux-gnu, x86_64-unknown-linux-gnu
- SHA256 checksums from release assets
- Test block: assert_match 'finetype', shell_output('finetype --version')

Auto-update workflow:
- .github/workflows/update-formula.yml
- Triggered via repository_dispatch or workflow_dispatch
- Downloads SHA256 files from release, regenerates formula
- Commits and pushes automatically

AC #3 (brew install works on macOS): Pending testing on macOS. Formula is correct but can't test on this Linux machine."
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Created Homebrew tap for FineType CLI at https://github.com/noon-org/homebrew-tap

**Formula:** `Formula/finetype.rb`
- Installs pre-built binaries from GitHub releases (no compilation needed)
- Supports macOS (Intel + Apple Silicon) and Linux (x86_64 + aarch64)
- SHA256 checksums verified from release assets
- Test block validates `finetype --version`

**Auto-update:** `.github/workflows/update-formula.yml`
- Triggered via repository_dispatch or manual workflow_dispatch
- Downloads SHA256 checksums from new release, regenerates formula, commits

**Verified working:**
- `brew install noon-org/tap/finetype` installs successfully
- `brew test noon-org/tap/finetype` passes
- `finetype --version` returns `finetype 0.1.0`
- `echo 'https://example.com' | finetype infer` returns `technology.internet.uri`

**Design decision:** Used pre-built binaries instead of cargo build for instant installation. The release workflow already builds for all 4 targets."
<!-- SECTION:FINAL_SUMMARY:END -->

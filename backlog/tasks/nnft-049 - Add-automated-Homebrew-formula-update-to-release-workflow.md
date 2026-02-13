---
id: NNFT-049
title: Add automated Homebrew formula update to release workflow
status: Done
assignee:
  - '@nightingale'
created_date: '2026-02-13 12:02'
updated_date: '2026-02-13 21:01'
labels:
  - ci
  - homebrew
  - infrastructure
dependencies: []
priority: medium
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
The Homebrew formula at noon-org/homebrew-tap was not updated when v0.1.1 was released, requiring a manual update. Added an `update-homebrew` job to the release workflow that automatically downloads SHA256 files from the release, generates the formula with correct version/hashes, and pushes to the homebrew-tap repository.

Requires a `HOMEBREW_TAP_TOKEN` secret to be configured in the finetype repository with write access to noon-org/homebrew-tap.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 update-homebrew job added to release.yml
- [x] #2 Job downloads SHA256 files from release artifacts
- [x] #3 Job generates correct formula with new version and hashes
- [x] #4 Job pushes updated formula to noon-org/homebrew-tap
- [x] #5 HOMEBREW_TAP_TOKEN secret configured in finetype repo
<!-- AC:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Workflow job added to release.yml. Still needs HOMEBREW_TAP_TOKEN secret to be configured manually by Hugh — this is a GitHub PAT with write access to noon-org/homebrew-tap.

HOMEBREW_TAP_TOKEN configured as org-level secret (noon-org) visible to all repositories. No per-repo secret needed.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Added `update-homebrew` job to `.github/workflows/release.yml` that runs after the release job completes. The job:
1. Extracts version from the git tag
2. Downloads SHA256 checksum files from the release
3. Parses SHA256 values for all four platform targets
4. Checks out `noon-org/homebrew-tap` using `HOMEBREW_TAP_TOKEN`
5. Generates the formula with correct version, URLs, and hashes
6. Commits and pushes the updated formula

The `HOMEBREW_TAP_TOKEN` is configured as an org-level secret on noon-org, visible to all repositories.

Files changed:
- .github/workflows/release.yml (new update-homebrew job)")
<!-- SECTION:FINAL_SUMMARY:END -->

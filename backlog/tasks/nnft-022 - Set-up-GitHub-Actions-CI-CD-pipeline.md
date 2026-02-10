---
id: NNFT-022
title: Set up GitHub Actions CI/CD pipeline
status: Done
assignee:
  - '@nightingale'
created_date: '2026-02-10 10:40'
updated_date: '2026-02-10 12:37'
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
- [x] #1 CI workflow runs cargo fmt, clippy, test, and finetype check on every push and PR
- [x] #2 No model training occurs in CI/CD — training is local hardware only
- [x] #3 Release workflow builds binaries for Linux x86_64, Linux aarch64, macOS x86_64, macOS aarch64
- [x] #4 Release workflow creates GitHub Release with attached binaries and SHA256 checksums
- [x] #5 CI passes on current main branch
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Check if .github/workflows/ exists\n2. Create CI workflow: fmt, clippy, test, finetype check\n3. Create Release workflow: cross-compile for 4 targets, create GH release with checksums\n4. Verify CI workflow would pass locally (run fmt, clippy, test)\n5. Commit workflow files
<!-- SECTION:PLAN:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Added GitHub Actions CI/CD pipeline with two workflows.\n\n**CI workflow** (`.github/workflows/ci.yml`) — runs on every push and PR to main:\n- `cargo fmt --all -- --check` (formatting gate)\n- `cargo clippy --all -- -D warnings` (lint gate, warnings-as-errors)\n- `cargo test --all` (33 tests across 3 crates)\n- `finetype check --verbose` (taxonomy checker: 151/151 definitions, all samples validated)\n- Uses `Swatinem/rust-cache@v2` for dependency caching\n\n**Release workflow** (`.github/workflows/release.yml`) — triggers on version tags (`v*`):\n- Cross-compiles for 4 targets: x86_64-linux, aarch64-linux (via cross), x86_64-darwin, aarch64-darwin\n- Packages each as `.tar.gz` with SHA256 checksums (ready for Homebrew formula)\n- Creates GitHub Release with auto-generated release notes and all artifacts\n\n**Code quality fixes** applied to pass CI cleanly:\n- `cargo fmt` formatting corrections in main.rs\n- 6x `push_str(\"\\n\")` → `push('\\n')` in checker.rs (clippy: single_char_add_str)\n- Derived `Default` for `Designation` enum in taxonomy.rs (clippy: derivable_impls)\n- 2x manual ceiling division → `.div_ceil()` in training modules (clippy: manual_div_ceil)\n- `filter_map(|l| l.ok())` → `map_while(Result::ok)` on stdin lines (clippy: lines_filter_map_ok)\n- 2x `&chunk.to_vec()` → `chunk` (clippy: unnecessary_to_owned)\n\nAll 5 CI gates verified locally before commit."}
</invoke>
<!-- SECTION:FINAL_SUMMARY:END -->

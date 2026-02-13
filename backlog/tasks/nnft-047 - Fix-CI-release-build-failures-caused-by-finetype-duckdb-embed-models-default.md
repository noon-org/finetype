---
id: NNFT-047
title: Fix CI/release build failures caused by finetype-duckdb embed-models default
status: Done
assignee:
  - '@nightingale'
created_date: '2026-02-13 10:58'
updated_date: '2026-02-13 10:58'
labels:
  - bugfix
  - ci
  - release
dependencies: []
priority: high
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
All four release build jobs failed on v0.1.1 tag push because `cargo build --release` built the entire workspace including `finetype-duckdb`, whose `build.rs` panics when model files aren't present (they aren't committed to git).

Root cause: `finetype-duckdb` has `embed-models` as a default feature, requiring `models/char-cnn-v2/` at compile time. CI runners don't have these files.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 Add default-members to workspace Cargo.toml excluding finetype-duckdb
- [x] #2 Update CI workflow: cargo test and cargo clippy use default members (no --all)
- [x] #3 Update release workflow: explicitly build -p finetype-cli
- [x] #4 Fix pre-existing cargo fmt issues
- [x] #5 Verify all CI jobs pass locally: fmt, clippy, test, smoke
<!-- AC:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Fixed all four release build failures caused by `finetype-duckdb` build.rs panicking in CI (no model files).

**Root cause:** `cargo build --release` and `cargo test --all` built the entire workspace. The DuckDB extension crate requires model files at compile time (via `embed-models` default feature), which aren't in the git repo.

**Changes:**
- `Cargo.toml` — Added `default-members` excluding `finetype-duckdb`. The DuckDB extension must be built explicitly with `cargo build -p finetype_duckdb`.
- `.github/workflows/ci.yml` — Changed `cargo clippy --all` → `cargo clippy` and `cargo test --all` → `cargo test` to use default-members.
- `.github/workflows/release.yml` — Added `-p finetype-cli` to both `cargo build` and `cross build` commands for explicit targeting.
- Fixed pre-existing `cargo fmt` issues in `column.rs` and `inference.rs`.

All jobs verified locally: fmt ✓, clippy ✓, test (62 passed) ✓, smoke (23 passed) ✓.
<!-- SECTION:FINAL_SUMMARY:END -->

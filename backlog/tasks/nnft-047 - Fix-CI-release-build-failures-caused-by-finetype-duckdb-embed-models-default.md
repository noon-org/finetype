---
id: NNFT-047
title: Fix CI/release build failures caused by finetype-duckdb embed-models default
status: Done
assignee:
  - '@nightingale'
created_date: '2026-02-13 10:58'
updated_date: '2026-02-13 11:16'
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
Fixed all CI and release build failures. Two rounds of fixes were needed:

**Round 1:** `finetype-duckdb` build.rs panicked because model files aren't in git.
- Added `default-members` to workspace Cargo.toml excluding `finetype-duckdb`
- CI: dropped `--all` from clippy/test; release: added `-p finetype-cli`
- Fixed pre-existing cargo fmt issues

**Round 2:** `finetype-cli` build.rs also panicked — same root cause (model files gitignored, hosted on HuggingFace).
- Added \"Download model from HuggingFace\" step to all CI/release jobs that compile `finetype-cli`
- Downloads `model.safetensors`, `labels.json`, `config.yaml` from `noon-org/finetype-char-cnn` repo (~340KB total)

**Result:** CI (5 jobs) and Release (4 builds + release) all pass. v0.1.1 binaries published for x86_64-linux, aarch64-linux, x86_64-darwin, aarch64-darwin.

**Files changed:**
- `Cargo.toml` — added default-members
- `.github/workflows/ci.yml` — model download + default-members flags + smoke test job
- `.github/workflows/release.yml` — model download + explicit `-p finetype-cli`
- `crates/finetype-model/src/column.rs` — cargo fmt
- `crates/finetype-model/src/inference.rs` — cargo fmt"
<!-- SECTION:FINAL_SUMMARY:END -->

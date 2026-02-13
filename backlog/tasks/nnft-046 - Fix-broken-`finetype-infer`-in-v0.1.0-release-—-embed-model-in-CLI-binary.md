---
id: NNFT-046
title: Fix broken `finetype infer` in v0.1.0 release — embed model in CLI binary
status: Done
assignee:
  - '@nightingale'
created_date: '2026-02-13 10:47'
updated_date: '2026-02-13 10:47'
labels:
  - bugfix
  - cli
  - release
dependencies: []
priority: high
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
The v0.1.0 release binary shipped with a broken `finetype infer` command. On macOS via Homebrew install, running `finetype infer -i "..."` failed with:

```
Error: Taxonomy error: Failed to read taxonomy file: No such file or directory
```

Root cause: `CharClassifier::load()` defaults to reading from `models/default/` directory, which doesn't exist on user machines — only in the development workspace. The release binary had no embedded model fallback.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 Add `embed-models` feature flag to finetype-cli
- [x] #2 Create build.rs that embeds CharCNN model (weights, labels, config) via include_bytes!
- [x] #3 Add `load_char_classifier()` helper that falls back to embedded model when path doesn't exist
- [x] #4 Replace direct `CharClassifier::load()` calls with fallback helper
- [x] #5 Tag v0.1.1 with fix
<!-- AC:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Fixed broken `finetype infer` in v0.1.0 release. The CLI binary couldn't classify anything because it tried to load model files from `models/default/` which doesn't exist outside the dev workspace.

**Changes:**
- Added `embed-models` feature (default on) to `finetype-cli`
- Created `crates/finetype-cli/build.rs` that generates `include_bytes!` for model.safetensors, labels.json, config.yaml from `models/char-cnn-v2/`
- Added `embedded` module in main.rs that includes the generated code at compile time
- Created `load_char_classifier()` helper: tries filesystem path first, falls back to embedded bytes
- Replaced `CharClassifier::load(&model)` calls in `cmd_infer` with `load_char_classifier(&model)`
- Bumped workspace version to 0.1.1

**Files changed:**
- `crates/finetype-cli/Cargo.toml` — added embed-models feature
- `crates/finetype-cli/build.rs` — NEW, compile-time model embedding
- `crates/finetype-cli/src/main.rs` — embedded module + fallback loader
- `Cargo.toml` — version 0.1.0 → 0.1.1

Tagged v0.1.1. This bug directly motivated NNFT-043 (CLI smoke tests).
<!-- SECTION:FINAL_SUMMARY:END -->

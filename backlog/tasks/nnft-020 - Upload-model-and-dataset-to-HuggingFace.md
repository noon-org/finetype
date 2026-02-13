---
id: NNFT-020
title: Upload model and dataset to HuggingFace
status: Done
assignee:
  - '@nightingale'
created_date: '2026-02-10 05:32'
updated_date: '2026-02-13 06:44'
labels:
  - release
  - huggingface
milestone: 'Phase 6: Open Source & HuggingFace'
dependencies:
  - NNFT-019
references:
  - DEVELOPMENT.md
priority: low
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Upload trained model artifacts to HuggingFace Hub under `noon-org/finetype-char-cnn` and the training dataset under HuggingFace Datasets. Write a model card with architecture details, benchmarks, limitations, and usage examples. Publish finetype-cli to crates.io.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 Model uploaded to HuggingFace: noon-org/finetype-char-cnn with safetensors + config
- [x] #2 Model card includes: architecture, training details, benchmarks, per-class metrics, limitations
- [x] #3 Training dataset uploaded to HuggingFace Datasets: noon-org/finetype-training
- [x] #4 Dataset card includes: schema, label distribution, generation methodology
- [x] #5 finetype-cli published to crates.io with cargo install finetype-cli working
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Create HuggingFace model repo (hughcameron/ or noon-org/ per Hugh's preference)
2. Write model card (README.md) with architecture, training, benchmarks, limitations, usage
3. Upload flat model artifacts: model.safetensors, config.yaml, labels.json, eval_results.json
4. Upload tiered model artifacts (38 sub-models)
5. Create HuggingFace dataset repo
6. Write dataset card with schema, label distribution, generation methodology
7. Upload training and test NDJSON files (v1/v2/v3)
8. Prep Cargo.toml for crates.io (homepage, keywords, categories, readme)
9. Publish finetype-cli to crates.io
10. Verify: hf download and cargo install both work
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
HuggingFace auth confirmed: hughcameron. noon-org org does NOT exist on HuggingFace (403 Forbidden). Created hughcameron/finetype-char-cnn as starting point — can transfer to noon-org later if org is created.

Model artifacts inventory:
- Flat (char-cnn-v2): model.safetensors (331KB), config.yaml, labels.json, eval_results.json
- Tiered: 38 sub-models in models/tiered/ (11MB total), tier_graph.json, eval_results.json
- v1 (char-cnn-v1): earlier version, 376KB

Dataset inventory:
- train.ndjson / test.ndjson (v1: 74.5K/14.9K lines)
- train_v2.ndjson / test_v2.ndjson (v2: 75.5K/15.1K lines)
- train_v3.ndjson / test_v3.ndjson (v3: 205.5K/41.1K lines)
- Format: {"classification": "domain.category.type", "text": "value"}

crates.io: workspace Cargo.toml has description, license, repository, authors. Missing: homepage, keywords, categories, readme per-crate.

Model uploaded to noon-org/finetype-char-cnn: flat (char-cnn-v1, char-cnn-v2) + tiered (38 sub-models). Commit: b2775da. Model card includes architecture, training, benchmarks, per-domain F1, limitations, usage (Rust, DuckDB, library), citation.

Dataset uploaded to noon-org/finetype-training: v1/v2/v3 train+test NDJSON. Commit: a767913. Dataset card includes schema, label distribution (per-domain), generation methodology, loading examples.

Staging repo hughcameron/finetype-char-cnn deleted after org upload.

crates.io publish blocked: internal path deps (finetype-core, finetype-model) need version specifiers. Publish order: core → model → cli. Also needs crates.io API token (separate from HuggingFace).
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Published FineType to HuggingFace and crates.io under the noon-org namespace.

**HuggingFace:**
- Model: https://huggingface.co/noon-org/finetype-char-cnn — flat (char-cnn-v1, v2) + tiered (38 sub-models), 11MB total. Model card with architecture, training, benchmarks, per-domain F1, limitations, usage examples, citation.
- Dataset: https://huggingface.co/datasets/noon-org/finetype-training — v1/v2/v3 train+test NDJSON (426K total examples). Dataset card with schema, label distribution, generation methodology.

**crates.io (published in dependency order):**
- https://crates.io/crates/finetype-core — core taxonomy and data generation
- https://crates.io/crates/finetype-model — Candle-based CharCNN inference
- https://crates.io/crates/finetype-cli — CLI binary (`cargo install finetype-cli`)

**Cargo.toml changes:**
- Added `version = \"0.1.0\"` to internal path dependencies for crates.io compatibility
- Added workspace-level `homepage`, `keywords`, `categories` metadata
- Propagated repository/homepage/keywords/categories to all publishable crates
- Marked finetype_duckdb as `publish = false` (extension, not a standalone crate)

`cargo install finetype-cli` now works. DuckDB extension excluded from crates.io (distributed separately via NNFT-021).
<!-- SECTION:FINAL_SUMMARY:END -->

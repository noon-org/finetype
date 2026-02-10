---
id: NNFT-020
title: Upload model and dataset to HuggingFace
status: To Do
assignee: []
created_date: '2026-02-10 05:32'
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
- [ ] #1 Model uploaded to HuggingFace: noon-org/finetype-char-cnn with safetensors + config
- [ ] #2 Model card includes: architecture, training details, benchmarks, per-class metrics, limitations
- [ ] #3 Training dataset uploaded to HuggingFace Datasets: noon-org/finetype-training
- [ ] #4 Dataset card includes: schema, label distribution, generation methodology
- [ ] #5 finetype-cli published to crates.io with cargo install finetype-cli working
<!-- AC:END -->

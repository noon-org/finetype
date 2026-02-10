---
id: NNFT-003
title: Build CharCNN training and inference pipeline
status: Done
assignee: []
created_date: '2026-02-10 05:29'
labels:
  - model
  - training
  - infrastructure
milestone: 'Phase 3: Build & Train'
dependencies: []
references:
  - crates/finetype-model/src/char_cnn.rs
  - crates/finetype-model/src/char_training.rs
  - crates/finetype-model/src/inference.rs
  - crates/finetype-cli/src/main.rs
priority: high
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Implement a character-level CNN model in finetype-model using Candle. Build the full training loop (data loading, forward pass, loss, backprop) and inference pipeline (model loading, labels.json mapping, confidence scoring). Wire into CLI with --model-type char-cnn flag for both train and infer subcommands.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 CharCNN model architecture implemented (embed → conv → pool → FC → softmax)
- [ ] #2 Training loop with loss computation and backpropagation works end-to-end
- [ ] #3 labels.json saved during training and loaded during inference
- [ ] #4 CLI supports --model-type char-cnn for both train and infer commands
- [ ] #5 Proof of concept: 2-epoch training reaches >80% accuracy
<!-- AC:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Built the complete CharCNN pipeline in finetype-model using Candle. Architecture: char embedding (vocab=97, dim=32) → conv1d (64 filters) → global max pool → FC (128 hidden) → 151-class output. Training loop handles NDJSON data loading, cross-entropy loss, and SGD optimization. The CLI exposes --model-type char-cnn. A 2-epoch proof of concept reached 84% accuracy, validating the approach.
<!-- SECTION:FINAL_SUMMARY:END -->

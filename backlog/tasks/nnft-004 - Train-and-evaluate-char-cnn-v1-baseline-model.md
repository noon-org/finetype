---
id: NNFT-004
title: Train and evaluate char-cnn-v1 baseline model
status: Done
assignee: []
created_date: '2026-02-10 05:30'
labels:
  - model
  - evaluation
milestone: 'Phase 3: Build & Train'
dependencies: []
references:
  - models/char-cnn-v1/model.safetensors
  - models/char-cnn-v1/config.yaml
  - models/char-cnn-v1/labels.json
  - models/char-cnn-v1/eval_results.json
priority: high
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Train the CharCNN on the full 149-label dataset and evaluate on the held-out test set. Produce per-class precision/recall/F1, top-3 accuracy, and a confusion matrix of the worst-performing class pairs. Save model artifacts (model.safetensors, config.yaml, labels.json, eval_results.json).
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Model trained on 74,500 training samples across 149 labels
- [ ] #2 Evaluation run on 14,900 test samples with per-class metrics
- [ ] #3 Model artifacts saved: model.safetensors, config.yaml, labels.json, eval_results.json
- [ ] #4 Baseline metrics documented: overall accuracy, top-3 accuracy, top confusion pairs
<!-- AC:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Trained char-cnn-v1 on the full 149-label dataset. Results: 89.8% accuracy, 98.5% top-3 accuracy (14,900 test samples). Average confidence on correct predictions: 95.5%. Key confusion patterns identified: gender_symbol↔emoji (100%), iso_8601_offset↔rfc_3339 (99%), credit_card_number↔imei (83%). These confusions inform targeted generator and taxonomy improvements for the next iteration.
<!-- SECTION:FINAL_SUMMARY:END -->

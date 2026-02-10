---
id: NNFT-009
title: Train char-cnn-v2 with improved generators and extended epochs
status: To Do
assignee: []
created_date: '2026-02-10 05:31'
labels:
  - model
  - training
milestone: 'Phase 3: Build & Train'
dependencies:
  - NNFT-005
  - NNFT-008
references:
  - crates/finetype-model/src/char_training.rs
  - models/char-cnn-v1/eval_results.json
priority: high
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
After fixing generator quality issues (NNFT-005) and resolving taxonomy ambiguities (NNFT-008), regenerate datasets and train a new CharCNN model with more epochs to establish the improved baseline. Target: >93% accuracy on the flat model (up from 89.8%).
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Regenerated training data with fixed generators (500 samples/label)
- [ ] #2 Regenerated test data with fixed generators (100 samples/label, different seed)
- [ ] #3 Model trained for 10+ epochs with learning rate schedule
- [ ] #4 Overall accuracy > 93% on test set (up from 89.8%)
- [ ] #5 Previously confused classes show measurable improvement (gender_symbol, credit_card, pin, port)
- [ ] #6 Model artifacts saved as models/char-cnn-v2/
- [ ] #7 Evaluation comparison: v1 vs v2 per-class metrics
<!-- AC:END -->

---
id: NNFT-011
title: Design and implement tiered inference models
status: To Do
assignee: []
created_date: '2026-02-10 05:31'
labels:
  - model
  - architecture
  - training
milestone: 'Phase 3: Build & Train'
dependencies:
  - NNFT-009
references:
  - crates/finetype-model/src/inference.rs
  - crates/finetype-model/src/char_cnn.rs
  - DEVELOPMENT.md
priority: high
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Replace the single flat 149-class model with a graph of small, specialised models following the tiered architecture described in DEVELOPMENT.md:

- **Tier 0**: Broad type classifier (7-10 classes: TIMESTAMP, DATE, TIME, BIGINT, VARCHAR, BOOLEAN, UUID, INTERVAL, JSON)
- **Tier 1**: Per-broad-type category models (e.g., VARCHAR → internet/person/code/payment/...)
- **Tier 2**: Per-category type models (e.g., internet → ip_v4/ip_v6/mac_address/url/...)

Each tier model is a smaller CharCNN trained independently. The `tier` field in definitions already encodes the graph structure. Implement the tiered inference engine that chains models, passing each value through Tier 0 → Tier 1 → Tier 2.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Tier 0 model trained: broad type classification with >98% accuracy
- [ ] #2 Tier 1 models trained for each broad type category
- [ ] #3 Tier 2 models trained for categories with >5 types
- [ ] #4 Tiered inference engine chains models correctly: Tier 0 → Tier 1 → Tier 2
- [ ] #5 Overall tiered accuracy exceeds flat model accuracy
- [ ] #6 Each tier model can be retrained independently without affecting others
- [ ] #7 CLI supports tiered inference via --model-type tiered or auto-detection
<!-- AC:END -->

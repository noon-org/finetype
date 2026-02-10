---
id: NNFT-002
title: Implement synthetic data generators for all 151 types
status: Done
assignee: []
created_date: '2026-02-10 05:29'
labels:
  - data-generation
  - foundation
milestone: 'Phase 2: Data Generation'
dependencies: []
references:
  - crates/finetype-core/src/generator.rs
  - crates/finetype-core/src/checker.rs
  - data/train.ndjson
  - data/test.ndjson
priority: high
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Build a Generator in finetype-core that produces realistic synthetic samples for every type in the taxonomy. Each generator match arm must correspond 1:1 with a YAML definition key. Generate training data (500 samples/label) and test data (100 samples/label, different seed). Add workspace dependencies: fake, base64, sha2, md-5, glob.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Generator produces samples for all 151 types (149 active labels)
- [ ] #2 1:1 alignment between generator match arms and YAML definition keys verified by checker
- [ ] #3 Training dataset: 74,500 samples (500/label × 149 labels, priority ≥ 1)
- [ ] #4 Test dataset: 14,900 samples (100/label × 149 labels, different seed)
- [ ] #5 cargo test --all passes including checker validation (151/151)
<!-- AC:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Implemented the Generator with synthetic data production for all 151 taxonomy types. The checker CLI validates 1:1 alignment between generators and YAML definitions (151/151 pass). Generated 74,500 training samples and 14,900 test samples as NDJSON. Added fake, base64, sha2, md-5, glob as workspace dependencies.
<!-- SECTION:FINAL_SUMMARY:END -->

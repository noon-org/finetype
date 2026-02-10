---
id: NNFT-025
title: Evaluate model against GitTables real-world dataset
status: To Do
assignee: []
created_date: '2026-02-10 10:40'
labels:
  - evaluation
  - data-quality
  - research
milestone: 'Phase 3: Build & Train'
dependencies:
  - NNFT-009
references:
  - 'https://gittables.github.io/'
  - models/char-cnn-v1/eval_results.json
priority: medium
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
All evaluation to date uses synthetic data from our own generators. Before shipping the DuckDB extension, validate against real-world column data from GitTables (https://gittables.github.io/) — a corpus of 1M relational tables extracted from CSV files on GitHub, with semantic type annotations from DBpedia and Schema.org.

**Approach:**
1. Download the GitTables benchmark subset (3.6 MB) or a topic slice
2. Sample columns across diverse domains
3. Run FineType inference on sampled column values
4. Compare FineType predictions against GitTables semantic type annotations
5. Identify systematic gaps: types that exist in real data but aren't in our taxonomy
6. Measure accuracy on real-world data vs synthetic test set

This provides ground truth for how the model performs outside its own training distribution.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 GitTables benchmark subset downloaded and accessible
- [ ] #2 At least 1,000 real-world columns sampled across diverse domains
- [ ] #3 FineType predictions compared against GitTables semantic type annotations
- [ ] #4 Accuracy on real-world data measured and compared to synthetic test set accuracy
- [ ] #5 Systematic gaps documented: real-world types missing from taxonomy
- [ ] #6 Report produced with per-domain breakdown of real-world performance
<!-- AC:END -->

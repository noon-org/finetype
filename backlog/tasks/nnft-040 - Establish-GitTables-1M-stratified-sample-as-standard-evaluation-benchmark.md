---
id: NNFT-040
title: Establish GitTables 1M stratified sample as standard evaluation benchmark
status: To Do
assignee:
  - '@nightingale'
created_date: '2026-02-13 10:10'
labels:
  - evaluation
  - infrastructure
  - gittables
dependencies:
  - NNFT-037
priority: low
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
The GitTables 1M evaluation (NNFT-037) showed the original benchmark subset (1,101 tables) was not fully representative — it over-represented difficult semantic types. The 1M stratified sample (50 tables/topic, 4,380 total) provides a more balanced evaluation.

Formalize this as the standard benchmark: reproducible sampling, pre-extracted metadata and values, documented evaluation script, and baseline metrics for comparison across model versions.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Pre-extracted evaluation data committed to repo or hosted for reproducibility
- [ ] #2 Evaluation script (eval_1m.sql) documented with clear usage instructions
- [ ] #3 Baseline metrics recorded: 55.3% domain accuracy, per-domain breakdown
- [ ] #4 CI or Makefile target to re-run evaluation after model changes
- [ ] #5 REPORT.md updated to designate 1M sample as primary benchmark
<!-- AC:END -->

---
id: NNFT-041
title: Add per-topic evaluation harnesses for domain-specific accuracy tracking
status: To Do
assignee:
  - '@nightingale'
created_date: '2026-02-13 10:10'
labels:
  - evaluation
  - infrastructure
dependencies:
  - NNFT-037
priority: low
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
The GitTables 1M evaluation covers 94 topics but currently only reports aggregate domain accuracy. Per-topic harnesses would enable tracking accuracy for specific data domains (e.g., healthcare, finance, geography) and identify where FineType performs best/worst.

This supports targeted model improvements and helps users understand FineType's strengths for their specific use case.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Per-topic accuracy report generated from eval_1m.sql output
- [ ] #2 Top 10 and bottom 10 topics by accuracy identified and documented
- [ ] #3 Topic-level confusion matrices available for worst-performing topics
- [ ] #4 Results integrated into REPORT.md or separate per-topic analysis
<!-- AC:END -->

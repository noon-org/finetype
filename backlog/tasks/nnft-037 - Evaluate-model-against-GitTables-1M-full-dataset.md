---
id: NNFT-037
title: Evaluate model against GitTables 1M full dataset
status: To Do
assignee: []
created_date: '2026-02-13 05:31'
labels:
  - evaluation
  - data-quality
  - gittables
dependencies:
  - NNFT-025
  - NNFT-028
references:
  - /home/hugh/git-tables/files-archive.zip
  - eval/gittables/eval.sql
  - eval/gittables/REPORT.md
  - crates/finetype-cli/src/main.rs
priority: medium
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
The GitTables 1M dataset (16GB, 96 topic zips, ~1M tables) is now available at /home/hugh/git-tables/files-archive.zip. Previous evaluation (NNFT-025, NNFT-028) used the small benchmark subset (1,101 tables, 2,384 columns). This task scales up to the full corpus for comprehensive real-world validation.

Goals:
1. Extract and catalog the 96 topic archives
2. Run column-mode evaluation across a statistically significant sample (or full corpus if feasible)
3. Measure accuracy, identify new confusion patterns, and stress-test inference throughput at scale
4. Compare results against the benchmark subset to validate whether the small sample was representative
5. Identify domain-specific strengths/weaknesses across the 96 topic categories

The existing eval-gittables CLI subcommand and eval.sql infrastructure may need adaptation for the larger dataset structure (nested topic zips vs flat table directory).
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 96 topic archives extracted and cataloged (table counts, column counts, total size)
- [ ] #2 Evaluation pipeline handles the nested topic-zip structure of GitTables 1M
- [ ] #3 At least 10,000 columns evaluated with column-mode inference (or full corpus if performance allows)
- [ ] #4 Per-topic accuracy breakdown produced (96 categories)
- [ ] #5 Throughput benchmarked: columns/second and values/second at scale
- [ ] #6 Results compared against NNFT-025 benchmark subset — representativeness assessed
- [ ] #7 New confusion patterns or taxonomy gaps documented beyond those found in benchmark subset
- [ ] #8 Updated REPORT.md with GitTables 1M findings
<!-- AC:END -->

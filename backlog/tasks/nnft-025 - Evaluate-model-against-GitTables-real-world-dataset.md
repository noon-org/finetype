---
id: NNFT-025
title: Evaluate model against GitTables real-world dataset
status: Done
assignee:
  - '@nightingale'
created_date: '2026-02-10 10:40'
updated_date: '2026-02-10 16:49'
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
- [x] #1 GitTables benchmark subset downloaded and accessible
- [x] #2 At least 1,000 real-world columns sampled across diverse domains
- [x] #3 FineType predictions compared against GitTables semantic type annotations
- [x] #4 Accuracy on real-world data measured and compared to synthetic test set accuracy
- [x] #5 Systematic gaps documented: real-world types missing from taxonomy
- [x] #6 Report produced with per-domain breakdown of real-world performance
<!-- AC:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
## Implementation Notes

**2026-02-11 02:38** — Downloaded GitTables benchmark from Zenodo (record/5706316):
- 1,101 CSV tables (tables.zip → tables/tables/GitTables_*.csv)
- Ground truth: schema_gt.csv (Schema.org), dbpedia_gt.csv (DBpedia)
- Label dictionaries: schema_labels.csv (59 types), dbpedia_labels.csv (275 types)

**2026-02-11 02:40** — Wrote eval.sql DuckDB evaluation script:
- Loads ground truth, preferring Schema.org over DBpedia
- Reads all 1,101 CSVs with union_by_name, UNPIVOTs to (table_file, col_idx, value)
- Joins with ground truth, samples up to 20 values per column
- Runs finetype() classification via DuckDB extension
- Majority vote per column, domain-level accuracy analysis

**2026-02-11 02:43** — Fixed three issues:
1. `filename` column caught in UNPIVOT → excluded with `* EXCLUDE (filename)`
2. `column0` vs `col0` naming → used `regexp_extract(col_name, 'col(\d+)', 1)`
3. Re-run failures → changed all to `CREATE OR REPLACE TABLE`

**2026-02-11 02:45** — Evaluation completed in 49 seconds:
- 2,384 annotated columns across 891 tables
- 34,449 values classified, 86 unique FineType labels
- Format-detectable types: URLs 90%, timestamps 100%, dates 88%, countries 100%, names 80%
- Overall domain accuracy: 42.2% (expected — GitTables annotates semantics, FineType detects format)

**2026-02-11 02:47** — Wrote REPORT.md with full analysis and recommendations.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Evaluated FineType CharCNN model against the GitTables real-world benchmark — the first validation outside synthetic training data.

**Scale:** 2,384 annotated columns across 891 tables, 34,449 values classified in 49 seconds via the DuckDB extension.

**Key Results:**
- Format-detectable types achieve 85–100% accuracy on real data (URLs 90%, timestamps 100%, dates 88%, countries 100%, person names 80%), closely matching the 91.97% synthetic accuracy
- Overall domain-level accuracy is 42.2%, which is expected: GitTables annotates *semantic meaning* while FineType detects *data format* — these are fundamentally different tasks
- Technology domain: 95.6%, Geography: 71%, Identity: 52.3%, Datetime: 43.8%, Representation: 23.9%

**Gaps identified:**
- Semantic-only types (rank, genus, species, class, etc.) have no format signal — require NLP
- Year detection needs column-mode disambiguation (4-digit numbers are ambiguous)
- Postal codes and emails need larger real-world samples

**Artifacts:**
- `eval/gittables/eval.sql` — Reproducible DuckDB evaluation script
- `eval/gittables/REPORT.md` — Full analysis with per-domain breakdown and recommendations
- Ground truth CSVs from Schema.org and DBpedia ontologies committed for reproducibility
<!-- SECTION:FINAL_SUMMARY:END -->

---
id: NNFT-030
title: Update README and DEVELOPMENT.md to reflect current state
status: Done
assignee:
  - '@nightingale'
created_date: '2026-02-10 22:23'
updated_date: '2026-02-10 22:27'
labels:
  - documentation
dependencies: []
priority: medium
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
The README and DEVELOPMENT.md are significantly out of date. They don't reflect: DuckDB extension is implemented (says "planned"), 112 tests (says 38), column-mode inference, new CLI commands (profile, validate, eval-gittables, check), real-world GitTables evaluation, year disambiguation, locale-aware training data, or current model accuracy (92.50%). Update both documents to accurately describe the current project state.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 README reflects all current CLI commands (infer, generate, train, taxonomy, check, validate, profile, eval, eval-gittables)
- [x] #2 README test count updated to 112
- [x] #3 README DuckDB section shows all 5 functions (finetype, finetype_detail, finetype_cast, finetype_unpack, finetype_version)
- [x] #4 README performance section updated with char-cnn-v2 accuracy (92.50%)
- [x] #5 README mentions column-mode inference and real-world GitTables evaluation
- [x] #6 DEVELOPMENT.md deprecated — replaced with redirect to README and backlog
- [x] #7 README Architecture section covers all 4 crates including finetype-duckdb
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Update README.md comprehensively:
   - Features section: 112 tests, 5 DuckDB functions, column-mode inference
   - CLI section: all 9 commands with examples
   - DuckDB section: all 5 scalar functions (finetype, finetype_detail, finetype_cast, finetype_unpack, finetype_version)
   - Performance section: char-cnn-v2 92.50% accuracy, GitTables real-world results
   - Architecture section: 4 crates including finetype-duckdb (implemented, not planned)
   - Repository structure: updated to reflect current state
   - Development commands: updated with all commands
   - Link to backlog/ for roadmap and project management

2. Deprecate DEVELOPMENT.md:
   - Replace with brief redirect to README + backlog
   - Preserve taxonomy reference docs (definition schema) in README or separate doc
   - Architecture decisions stay in README (brief) + backlog/decisions/

3. Verify: cargo test, cargo fmt, cargo clippy
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Updated README.md with all 9 CLI commands, 112 tests, 5 DuckDB functions, 92.50% accuracy, column-mode inference section, GitTables evaluation results, 4-crate architecture, taxonomy definition schema example. Deprecated DEVELOPMENT.md with redirect to README + backlog + labels/ + eval/gittables/REPORT.md. Preserved Related Repositories section in DEVELOPMENT.md.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Updated README.md to comprehensively reflect the current project state and deprecated DEVELOPMENT.md in favour of backlog tasks.\n\nREADME changes:\n- Features: 112 tests (was 38), 5 DuckDB functions (was 2), column-mode inference, real-world validation\n- CLI: documented all 9 commands (infer, generate, train, taxonomy, check, validate, profile, eval, eval-gittables)\n- DuckDB: all 5 scalar functions with SQL examples (finetype, finetype_detail, finetype_cast, finetype_unpack, finetype_version)\n- Performance: 92.50% CharCNN v2 accuracy (was 91.97%), GitTables evaluation summary\n- New section: Column-Mode Inference explaining disambiguation rules\n- Architecture: 4 crates (was 3), finetype-duckdb shown as implemented with test counts per crate\n- Repository structure updated, development commands updated\n- Taxonomy definition schema example preserved from DEVELOPMENT.md\n- Links to backlog/ for project management\n\nDEVELOPMENT.md:\n- Replaced 425-line development plan with deprecation notice pointing to README, backlog/, labels/, and eval/gittables/REPORT.md\n- Preserved Related Repositories section
<!-- SECTION:FINAL_SUMMARY:END -->

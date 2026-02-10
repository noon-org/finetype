---
id: NNFT-014
title: Implement finetype validate CLI command with quality reports
status: To Do
assignee: []
created_date: '2026-02-10 05:31'
updated_date: '2026-02-10 06:50'
labels:
  - cli
  - validation
  - data-quality
milestone: 'Phase 4: Validation Engine'
dependencies:
  - NNFT-013
references:
  - crates/finetype-cli/src/main.rs
priority: medium
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Add a `finetype validate` CLI subcommand that takes a file of values + their inferred types and produces a data quality report. The report includes per-column validation stats, error distributions, and recommendations (quarantine, null, or manual review). Supports NDJSON and CSV output formats.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 finetype validate subcommand accepts input file with values and inferred types
- [ ] #2 Per-column quality report: valid %, invalid %, null %, top error patterns
- [ ] #3 Output formats: human-readable table (default), JSON, CSV
- [ ] #4 --strategy flag per column: quarantine (default), null, ffill, bfill
- [ ] #5 Quarantine writes invalid rows to separate file with row index and error reason
- [ ] #6 Null mode outputs cleaned data with invalid values replaced by NULL
- [ ] #7 ffill/bfill modes output cleaned data with forward/backward filled values
- [ ] #8 Exit code reflects data quality (0 = all valid, 1 = some invalid)
<!-- AC:END -->

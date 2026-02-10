---
id: NNFT-014
title: Implement finetype validate CLI command with quality reports
status: Done
assignee:
  - '@nightingale'
created_date: '2026-02-10 05:31'
updated_date: '2026-02-10 16:37'
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
- [x] #1 finetype validate subcommand accepts input file with values and inferred types
- [x] #2 Per-column quality report: valid %, invalid %, null %, top error patterns
- [x] #3 Output formats: human-readable table (default), JSON, CSV
- [x] #4 --strategy flag per column: quarantine (default), null, ffill, bfill
- [x] #5 Quarantine writes invalid rows to separate file with row index and error reason
- [x] #6 Null mode outputs cleaned data with invalid values replaced by NULL
- [x] #7 ffill/bfill modes output cleaned data with forward/backward filled values
- [x] #8 Exit code reflects data quality (0 = all valid, 1 = some invalid)
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Add `Validate` variant to `Commands` enum with CLI args:\n   - `--file` (required): input file path\n   - `--label`: optional label override (validates all values against one type)\n   - `--taxonomy`: taxonomy path (default: labels)\n   - `--strategy`: quarantine/null/ffill/bfill (default: quarantine)\n   - `--output`: plain/json/csv output format\n   - `--quarantine-file`: path for quarantined rows output\n   - `--cleaned-file`: path for cleaned data output\n2. Support two input modes:\n   - Plain text + `--label`: one value per line, all validated against same label\n   - NDJSON: each line has `{\"value\": \"...\", \"label\": \"...\"}`, grouped by label\n3. Implement `cmd_validate()`: load taxonomy, read input, group by label, call `validate_column_for_label()` per group\n4. Render per-column quality report: valid %, invalid %, null %, top error patterns\n5. Write quarantine file (NDJSON with row_index, value, errors) in quarantine mode\n6. Write cleaned data file in null/ffill/bfill modes\n7. Set exit code: 0 if all valid, 1 if any invalid\n8. Run cargo test, clippy, fmt
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Implemented finetype validate subcommand in CLI (crates/finetype-cli/src/main.rs).\n\nTwo input modes:\n- Plain text + --label: one value per line, all validated against same label\n- NDJSON: each line has value/label (or input/class) fields, grouped by label\n\nSupports all four strategies (quarantine/null/ffill/bfill) via --strategy flag.\nThree output formats (plain/json/csv) for the quality report.\nQuarantine file: NDJSON with row index, value, label, error messages.\nCleaned file: matches input format (plain text → plain text, NDJSON → NDJSON).\nExit code 0 when all valid, 1 when any invalid.\n\nTested against real taxonomy (technology.internet.ip_v4 with validation pattern).\nAll workspace tests pass, clippy clean, fmt clean.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Implemented `finetype validate` CLI subcommand for data quality validation against taxonomy schemas.\n\nChanges:\n- Added `ValidateStrategy` enum (quarantine/null/ffill/bfill) with clap ValueEnum\n- Added `Validate` command variant with full CLI args (file, label, taxonomy, strategy, output, quarantine-file, cleaned-file)\n- Implemented `cmd_validate()`: reads plain text or NDJSON input, groups by label, validates each group via `validate_column_for_label()`, outputs quality reports\n- Quality report shows per-column stats (valid/invalid/null counts and percentages, validity rate, top error patterns)\n- Three output formats: human-readable table, JSON, CSV\n- Quarantine mode writes invalid rows to NDJSON file with row index, value, label, and error messages\n- Null/ffill/bfill modes write cleaned data file preserving input format (plain text → plain text, NDJSON → NDJSON)\n- Exit code reflects data quality (0 = all valid, 1 = some invalid)\n- Compatible with `finetype infer` JSON output (supports both value/label and input/class field names)\n\nTests:\n- cargo test --workspace: 97 tests passing\n- cargo clippy --workspace: clean\n- Manual validation against real taxonomy (ip_v4 pattern) with all four strategies
<!-- SECTION:FINAL_SUMMARY:END -->

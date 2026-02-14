---
id: NNFT-059
title: Add Excel custom number format detection
status: To Do
assignee: []
created_date: '2026-02-14 10:08'
labels:
  - taxonomy
  - generator
  - feature
dependencies: []
references:
  - >-
    https://learn.microsoft.com/en-us/dotnet/standard/base-types/formatting-types
priority: low
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Detect Excel/spreadsheet custom number format strings that commonly appear in exported data. These format codes tell spreadsheet applications how to display numbers and are found in metadata, headers, or as literal strings in data exports.

Common Excel format patterns:
- Number: `#,##0.00`, `0.00`, `#,##0`
- Currency: `$#,##0.00`, `€#,##0.00`, `[$$-409]#,##0.00`
- Percentage: `0.00%`, `0%`
- Date: `mm/dd/yyyy`, `d-mmm-yy`, `dddd, mmmm dd, yyyy`
- Time: `h:mm:ss AM/PM`, `[h]:mm:ss`
- Scientific: `0.00E+00`
- Custom: `#,##0.00;[Red]-#,##0.00;0.00;"text"` (positive;negative;zero;text sections)

This is especially relevant for GitTables data (sourced from spreadsheets) where format strings may appear as column metadata.

Reference: https://learn.microsoft.com/en-us/dotnet/standard/base-types/formatting-types
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Excel number format string type added to taxonomy
- [ ] #2 Generator produces common Excel format patterns (number, currency, date, percentage)
- [ ] #3 Detection distinguishes format strings from regular text
- [ ] #4 DuckDB transformation contract documented (likely VARCHAR passthrough)
<!-- AC:END -->

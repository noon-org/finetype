---
id: NNFT-057
title: 'Add SI-prefix human-readable number detection (12.2K, 1.5M, 2.3B)'
status: To Do
assignee: []
created_date: '2026-02-14 10:08'
labels:
  - taxonomy
  - generator
  - feature
dependencies: []
references:
  - 'https://github.com/debrouwere/python-ballpark'
priority: low
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Add a new type for human-readable numbers with SI/business notation suffixes. These are extremely common in dashboards, reports, and spreadsheet data.

Formats to detect:
- K/k (thousands): "12.2K", "500k"
- M/m (millions): "1.5M", "3.2m"  
- B/b (billions): "2.3B"
- T/t (trillions): "1.1T"
- Optional currency prefix: "$1.5M", "€2.3B"
- Optional sign: "-500K", "+1.2M"

The DuckDB transformation contract would parse the suffix and multiply: "12.2K" → 12200.

Inspired by python-ballpark library which calls this "business notation."

Reference: https://github.com/debrouwere/python-ballpark
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 New type added for SI-prefix numbers (e.g., representation.numeric.si_number)
- [ ] #2 Generator produces K/M/B/T suffixed values with varied precision
- [ ] #3 DuckDB transformation contract parses suffix and converts to numeric value
- [ ] #4 Detection distinguishes SI numbers from plain text (e.g., 'OK' is not a number)
<!-- AC:END -->

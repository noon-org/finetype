---
id: NNFT-045
title: Review and decide on locale classification feature (4-level labels)
status: To Do
assignee:
  - '@nightingale'
created_date: '2026-02-13 10:40'
updated_date: '2026-02-14 10:03'
labels:
  - architecture
  - locale
  - decision
dependencies: []
references:
  - labels/definitions_datetime.yaml
  - crates/finetype-core/src/generator.rs
documentation:
  - 'https://duckdb.org/docs/sql/functions/dateformat.html'
priority: medium
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
NNFT-018 implemented locale-aware training data generation with 4-level labels (domain.category.type.LOCALE, e.g., identity.person.phone_number.EN_AU). Training data v3 has 411 unique 4-level labels across 16 locales.

The current production model (char-cnn-v3) uses 3-level labels (152 types). The locale feature was built for future tiered models but needs a decision on direction.

**Critical finding:** DuckDB's `strptime` only understands English month/day names. Non-English dates like `6 janvier 2025` fail with `strptime(col, '%d %B %Y')`. This means FineType's transformation contract — "if the model predicts a type, the DuckDB cast succeeds" — breaks for non-English locale data.

This affects any locale-specific type where the transform relies on textual month names, day names, or locale-specific formatting that DuckDB can't parse natively. The problem is not limited to dates — phone numbers, addresses, and currency formats also have locale-specific patterns that may need different transforms.

**Options:**
1. **Two-stage detection** — Detect format type (3-level) + locale separately, compose the right transform (e.g., translate "janvier"→"January" before strptime, or provide locale-specific format strings)
2. **Ship full 4-level classification** — Train on 400+ locale-specific labels, each with its own transform. Higher accuracy risk due to class fragmentation.
3. **Normalization layer** — Keep 3-level types but add a `finetype_cast()` normalization step that handles locale translation before DuckDB casting.
4. **Keep as infra, document limitation** — Current model only guarantees transforms for English-locale data. Document this and revisit later.

The decision should weigh: transformation contract integrity, model accuracy impact, implementation complexity, and real-world demand for non-English locale support.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Decision documented: which locale detection approach to pursue (two-stage, full 4-level, normalization layer, or defer)
- [ ] #2 DuckDB strptime locale limitation documented in project (README or docs)
- [ ] #3 If shipping: create follow-up tasks for locale-aware model and transform pipeline
- [ ] #4 If deferring: document which types are affected and add known-limitation note
- [ ] #5 README updated to reflect locale support status and any transform caveats
<!-- AC:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
**Investigation findings (Feb 14 2026):**

DuckDB `strptime` only accepts English month/day names:
- `strptime('6 January 2025', '%d %B %Y')` → ✅ `2025-01-06 00:00:00`
- `strptime('6 janvier 2025', '%d %B %Y')` → ❌ fails
- `try_strptime('6 janvier 2025', '%d %B %Y')` → NULL
- No DuckDB locale/language setting exists to change this (only `Calendar=gregorian`)

This breaks FineType's transformation contract for non-English date formats. If the model classifies a French date as `datetime.date.long_full_month`, the promised transform `strptime(col, '%d %B %Y')` will fail.

**Types affected:** Any type using `%B` (full month name), `%b` (abbreviated month), or `%A`/`%a` (day name) in the transform — primarily `datetime.date.long_full_month`, `datetime.date.abbreviated_month`, `datetime.date.weekday_full_month`, `datetime.date.weekday_abbreviated_month`, and similar timestamp variants.

**Current scope of locale generators:** 16 locales across identity (names, phones), datetime (date formats), and geography (addresses, postal codes) domains.
<!-- SECTION:NOTES:END -->

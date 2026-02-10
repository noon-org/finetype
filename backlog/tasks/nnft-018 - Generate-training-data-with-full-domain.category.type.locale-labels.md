---
id: NNFT-018
title: Generate training data with full domain.category.type.locale labels
status: Done
assignee:
  - '@nightingale'
created_date: '2026-02-10 05:32'
updated_date: '2026-02-10 17:07'
labels:
  - data-generation
  - locale
milestone: 'Phase 2: Data Generation'
dependencies:
  - NNFT-006
references:
  - crates/finetype-core/src/generator.rs
  - DEVELOPMENT.md
priority: low
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Expand the data generation pipeline to produce samples with full 4-level labels including locale (e.g., `identity.person.phone_number.EN_AU` instead of just `identity.person.phone_number`). This requires the phonenumber crate (NNFT-006) for phone numbers and locale-aware generation for other locale_specific types like month names, day names, and address formats. The expanded labels are needed for locale-aware tiered models.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 Generator produces samples with 4-level labels: domain.category.type.locale
- [x] #2 All locale_specific types generate per-locale samples for their defined locales
- [x] #3 Universal types generate with .UNIVERSAL suffix
- [x] #4 Generated data validates against taxonomy (each label resolves to a known definition + locale)
- [x] #5 Training and test datasets regenerated with locale-expanded labels
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
## Implementation Plan

### Overview
Expand `Generator` to produce 4-level labels (`domain.category.type.LOCALE`) for locale-specific types and `.UNIVERSAL` suffix for universal types. This requires:

1. **Add locale-specific data tables** to generator.rs — per-locale first/last names, country names, city names, month names, weekday names, street data, postal code formats
2. **Add `generate_value_with_locale()` method** that routes locale-specific types to the correct per-locale generator
3. **Modify `generate_all()` to expand locale variants** — for each locale_specific definition, iterate over its `locales` list and generate N samples per (label, locale) pair; for universal types, append `.UNIVERSAL`
4. **Update `Sample` struct** to include optional locale field
5. **Update CLI output** to emit 4-level labels in NDJSON output
6. **Validate generated labels** against taxonomy (definition + locale exists)
7. **Regenerate training/test datasets** with new labels
8. **Add tests** for locale-expanded generation

### Locale Data Scope (21 locale-specific types)
**Identity (6):** full_name, first_name, last_name, phone_number, username, nationality
**Geography (9):** country, continent, state, city, full_address, street_name, street_suffix, postal_code, country_calling_code
**Datetime (6):** abbreviated_month, full_month, weekday_abbreviated, weekday_full, weekday (component), month_name (component)

### Locale Coverage
Primary: EN, DE, FR, ES, IT, NL, PL, RU, JA, ZH, KO, AR
Regional EN: EN_AU, EN_GB, EN_CA, EN_US (share EN names but locale-specific formats like phone/postal)

### Steps
1. Add locale data module with per-locale name/place/date tables
2. Add `generate_value_with_locale()` dispatching to locale-aware generators
3. Modify `generate_all()` to expand locales, emit 4-level labels
4. Update Sample struct and CLI output format
5. Run `finetype check` to validate generated labels against taxonomy
6. Test with small sample, then full regeneration
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
## Implementation Complete (2026-02-11)

### New files
- `crates/finetype-core/src/locale_data.rs` (~900 lines): Per-locale data tables for 16 locales (EN, EN_AU, EN_GB, EN_CA, EN_US, DE, FR, ES, IT, NL, PL, RU, JA, ZH, KO, AR) covering first/last names, countries, continents, cities, states, nationalities, month names, weekday names, street data, phone/postal formats

### Modified files
- `generator.rs`: Added `locale: Option<String>` field, `generate_all_localized()` method, `current_locale()` helper, locale-routed `gen_phone_number()` (14 formats) and `gen_postal_code()` (12 formats), refactored all locale-specific generators to use locale_data
- `lib.rs`: Added `pub mod locale_data`
- `main.rs`: Added `--localized` CLI flag

### Results
- 411 unique 4-level labels (283 locale-specific + 128 universal)
- 21 locale-specific types × up to 16 locales each
- East Asian name order (JA/ZH/KO: LastFirst)
- All 104 tests pass, clippy clean
- `finetype check` validates 151/151 generators at 100%
- train_v3.ndjson: 205,500 samples, test_v3.ndjson: 41,100 samples
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Implemented locale-aware training data generation with 4-level labels (domain.category.type.LOCALE).\n\nChanges:\n- Added `locale_data.rs` module (~900 lines) with per-locale data tables for 16 locales covering names, geography, datetime, and address data\n- Refactored `generator.rs` to support locale-aware generation: added `generate_all_localized()` method, locale-routed phone number (14 formats) and postal code (12 formats) generators, and updated all 21 locale-specific type generators to use locale_data lookups\n- Added `--localized` CLI flag to `generate` command for 4-level label mode\n- East Asian name order support (JA/ZH/KO: LastFirst instead of First Last)\n- 5 new/updated tests for locale routing, locale-aware names, months, and full localized generation\n\nResults:\n- 411 unique 4-level labels (283 locale-specific + 128 universal) vs 151 3-level labels previously\n- train_v3.ndjson: 205,500 samples, test_v3.ndjson: 41,100 samples\n- All 104 tests pass, clippy clean, finetype check 151/151 generators at 100%\n\nAll locale_specific types (identity: 6, geography: 9, datetime: 6) generate per-locale samples; universal/broad types get .UNIVERSAL suffix.">
<!-- SECTION:FINAL_SUMMARY:END -->

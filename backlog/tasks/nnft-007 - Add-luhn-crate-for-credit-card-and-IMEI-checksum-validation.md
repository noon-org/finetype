---
id: NNFT-007
title: Add luhn crate for credit card and IMEI checksum validation
status: Done
assignee:
  - '@nightingale'
created_date: '2026-02-10 05:30'
updated_date: '2026-02-10 12:00'
labels:
  - data-generation
  - validation
milestone: 'Phase 2: Data Generation'
dependencies: []
references:
  - crates/finetype-core/src/generator.rs
  - 'https://github.com/pacak/luhn'
priority: medium
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Integrate the `luhn` crate (https://github.com/pacak/luhn) into finetype-core for generating credit card numbers and IMEI codes with valid Luhn checksums. This will improve model discrimination between credit_card_number and imei since real credit cards follow Luhn + specific IIN prefixes while IMEIs follow Luhn + TAC prefixes.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 luhn crate added as workspace dependency
- [x] #2 credit_card_number generator produces Luhn-valid numbers with correct IIN prefixes per network
- [x] #3 imei generator produces Luhn-valid 15-digit numbers with realistic TAC prefixes
- [x] #4 ean generator produces valid EAN-13 with correct check digit
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Add `luhn = \"3\"` (latest) to workspace dependencies in root Cargo.toml and finetype-core/Cargo.toml\n2. Implement `compute_luhn_check_digit()` helper in generator.rs that takes a digit prefix string and returns the full number with valid Luhn check digit appended\n3. Rewrite `credit_card_number` generator:\n   - Visa: prefix \"4\" + 14 random digits + Luhn check digit (16 total)\n   - Mastercard: prefix \"51\"-\"55\" + 13 random digits + check digit (16 total)\n   - Amex: prefix \"34\"/\"37\" + 12 random digits + check digit (15 total)\n4. Rewrite `imei` generator:\n   - Use realistic TAC prefixes (8-digit Type Allocation Codes from known manufacturers)\n   - Generate 6 random serial digits\n   - Compute Luhn check digit for the 15th digit\n5. Rewrite `ean` generator:\n   - EAN-13: generate 12 random digits with realistic GS1 prefixes, compute EAN check digit (different from Luhn — weighted sum mod 10)\n   - EAN-8: generate 7 random digits, compute EAN check digit\n6. Add tests: validate all generated credit cards, IMEIs, and EANs pass their respective check digit algorithms\n7. Run `cargo test --all` to verify no regressions\n8. Run `cargo build --release` to verify clean compilation
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
- luhn crate v1.0.1 added (API: `luhn::valid(&str) -> bool`, `luhn::checksum(&str) -> u8`)\n- Implemented `luhn_check_digit()` helper using standard Luhn algorithm directly (the crate is used for validation in tests)\n- Implemented `ean_check_digit()` helper using weighted-sum mod 10 (alternating weights 1 and 3)\n- credit_card_number: Visa (4xxx/16), Mastercard (51-55xx/16), Amex (34|37xx/15), Discover (6011xx/16) — all Luhn-valid\n- imei: 20 realistic TAC prefixes from Apple/Samsung/Google/Huawei/OnePlus/Sony/LG + 6 random serial + Luhn check = 15 digits\n- ean: EAN-13 with 22 GS1 country prefixes (US, France, Germany, Japan, UK, China, etc.) + EAN-8 — both with valid check digits\n- 4 new tests (100 iterations each): test_credit_card_luhn_valid, test_imei_luhn_valid, test_ean_check_digit_valid, test_credit_card_network_prefixes\n- All 31/31 tests pass
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Integrated the `luhn` crate (v1.0.1) into finetype-core and rewrote three identification code generators to produce industry-standard valid output.

Changes:
- Added `luhn = "1.0"` to workspace and finetype-core dependencies
- Implemented `luhn_check_digit()` helper using the standard Luhn/mod-10 algorithm
- Implemented `ean_check_digit()` helper using weighted-sum mod 10 (alternating weights 1 and 3)
- **credit_card_number**: Now generates Luhn-valid numbers with correct IIN prefixes — Visa (4xxx/16-digit), Mastercard (51-55xx/16), Amex (34|37xx/15), Discover (6011xx/16)
- **imei**: Now generates Luhn-valid 15-digit IMEIs with 20 realistic TAC prefixes from Apple, Samsung, Google, Huawei, OnePlus, Sony, and LG
- **ean**: Now generates GS1-compliant barcodes with 22 country prefixes and valid EAN check digits for both EAN-13 and EAN-8 formats

Tests:
- Added 4 new test functions (100 iterations each): `test_credit_card_luhn_valid`, `test_imei_luhn_valid`, `test_ean_check_digit_valid`, `test_credit_card_network_prefixes`
- All 31 tests pass, clean release build
<!-- SECTION:FINAL_SUMMARY:END -->

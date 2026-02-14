---
id: NNFT-052
title: >-
  Add finance domain — ISIN, CUSIP, SEDOL, SWIFT/BIC, LEI, and ISO 4217 currency
  codes
status: Done
assignee:
  - '@nightingale'
created_date: '2026-02-14 10:07'
updated_date: '2026-02-14 10:24'
labels:
  - taxonomy
  - generator
  - feature
dependencies: []
documentation:
  - 'https://en.wikipedia.org/wiki/ISO_4217'
priority: medium
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Add financial identifier types to the taxonomy and implement generators. These are all format-checkable with fixed patterns, check digits, or known code lists — ideal for FineType.

Finance identifiers:
- **ISIN** (International Securities Identification Number): 2-letter country + 9-char alphanumeric + 1 check digit (Luhn). Example: US0378331005
- **CUSIP** (Committee on Uniform Securities Identification Procedures): 9 chars (6 issuer + 2 issue + 1 check). US/Canada securities. Example: 037833100
- **SEDOL** (Stock Exchange Daily Official List): 7-char alphanumeric with weighted check digit. UK/Ireland securities. Example: 0263494
- **SWIFT/BIC** (Bank Identifier Code): 8 or 11 chars (4 bank + 2 country + 2 location + optional 3 branch). Example: DEUTDEFF
- **LEI** (Legal Entity Identifier): 20-char alphanumeric (4 LOU + 14 entity + 2 check via ISO 7064). Example: 529900T8BM49AURSDO55

Currency codes (from v1 legacy types):
- **ISO 4217 currency code**: 3-letter codes (USD, EUR, GBP, JPY). ~180 active codes.
- **Currency symbol**: Unicode symbols ($, €, £, ¥, ₹). Finite set.

These should likely live under a new `identity.finance` category or extend the existing `identity.payment` category.

Reference: https://en.wikipedia.org/wiki/ISO_4217
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 ISIN type added to taxonomy with validation pattern and Luhn check digit
- [x] #2 CUSIP type added with 9-char pattern and check digit algorithm
- [x] #3 SEDOL type added with 7-char pattern and weighted check digit
- [x] #4 SWIFT/BIC type added with 8/11-char pattern
- [x] #5 LEI type added with 20-char pattern and ISO 7064 check
- [x] #6 ISO 4217 currency_code type added with known code list
- [x] #7 Currency symbol type added with Unicode symbol set
- [x] #8 Generators produce valid identifiers with correct check digits
- [x] #9 All new types have DuckDB transformation contracts
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Add taxonomy definitions to identity domain (extend payment category with finance types)
   - ISIN, CUSIP, SEDOL → identity.payment (securities identifiers alongside credit cards)
   - SWIFT/BIC, LEI → identity.payment (financial institution identifiers)
   - currency_code, currency_symbol → identity.payment (financial formatting)
   - Each with validation pattern, DuckDB transform, tier, samples

2. Implement generators in gen_identity() payment match arm
   - ISIN: 2-letter country code + 9 alphanumeric + Luhn check digit (convert letters to numbers for Luhn)
   - CUSIP: 6 issuer + 2 issue + 1 check digit (custom weighted algorithm)
   - SEDOL: 6 alphanumeric + 1 weighted check digit (weights: 1,3,1,7,3,9)
   - SWIFT/BIC: 4 bank letters + 2 country letters + 2 location alphanumeric + optional 3 branch
   - LEI: 4 LOU digits + 14 entity alphanumeric + 2 ISO 7064 check digits
   - currency_code: pick from ISO 4217 list
   - currency_symbol: pick from known symbols

3. Add check digit helper methods
   - isin_check_digit(): Luhn on alpha-to-numeric converted string
   - cusip_check_digit(): weighted sum mod 10
   - sedol_check_digit(): weighted sum mod 10 (weights 1,3,1,7,3,9)
   - lei_check_digits(): ISO 7064 Mod 97-10

4. Add unit tests for check digit validity

5. Verify training data generation works for all new types
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
All 7 types implemented and tested:
- ISIN: Luhn check digit on alpha-to-numeric expansion. Verified against known values (Apple US0378331005, SAP DE0007164600).
- CUSIP: Custom weighted algorithm with alpha-to-numeric conversion.
- SEDOL: Weighted sum (1,3,1,7,3,9) with no-vowels constraint.
- SWIFT/BIC: 8 or 11 chars with realistic country codes. ~40% generate branch codes.
- LEI: ISO 7064 Mod 97-10 check digits (same as IBAN). LOU prefixes from real registries.
- Currency code: 40 ISO 4217 codes covering major world currencies.
- Currency symbol: 30 Unicode currency symbols.

73 unit tests pass (8 new finance-specific tests). Taxonomy count: 152→159 types.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Added 7 financial identifier and currency types to FineType taxonomy with generators and check digit validation.

Changes:
- **Taxonomy** (`labels/definitions_identity.yaml`): Added ISIN, CUSIP, SEDOL, SWIFT/BIC, LEI, currency_code, and currency_symbol under `identity.payment` category. Each has validation pattern, DuckDB transformation contract, decompose expressions, and sample values.
- **Generators** (`crates/finetype-core/src/generator.rs`): Implemented generators for all 7 types with correct check digit algorithms:
  - ISIN: Luhn on alpha-to-numeric expanded string (A=10..Z=35)
  - CUSIP: Custom weighted sum with doubling at odd positions
  - SEDOL: Weighted sum with weights [1,3,1,7,3,9], no vowels
  - SWIFT/BIC: 8/11-char format with 20 real country codes
  - LEI: ISO 7064 Mod 97-10 check digits with real LOU prefixes
  - Currency code: 40 ISO 4217 codes
  - Currency symbol: 30 Unicode currency symbols
- **Helper methods**: Added `isin_check_digit()`, `cusip_check_digit()`, `sedol_check_digit()`, `lei_check_digits()`, and `alpha_to_num()` utility
- **Tests**: 8 new unit tests including known-value verification against real Apple Inc and SAP SE ISINs

Type count: 152 → 159. All 73 unit tests pass.
<!-- SECTION:FINAL_SUMMARY:END -->

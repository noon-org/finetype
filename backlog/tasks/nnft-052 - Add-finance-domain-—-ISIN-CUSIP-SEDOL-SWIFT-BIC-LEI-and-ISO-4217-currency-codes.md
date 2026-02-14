---
id: NNFT-052
title: >-
  Add finance domain — ISIN, CUSIP, SEDOL, SWIFT/BIC, LEI, and ISO 4217 currency
  codes
status: To Do
assignee: []
created_date: '2026-02-14 10:07'
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
- [ ] #1 ISIN type added to taxonomy with validation pattern and Luhn check digit
- [ ] #2 CUSIP type added with 9-char pattern and check digit algorithm
- [ ] #3 SEDOL type added with 7-char pattern and weighted check digit
- [ ] #4 SWIFT/BIC type added with 8/11-char pattern
- [ ] #5 LEI type added with 20-char pattern and ISO 7064 check
- [ ] #6 ISO 4217 currency_code type added with known code list
- [ ] #7 Currency symbol type added with Unicode symbol set
- [ ] #8 Generators produce valid identifiers with correct check digits
- [ ] #9 All new types have DuckDB transformation contracts
<!-- AC:END -->

---
id: NNFT-007
title: Add luhn crate for credit card and IMEI checksum validation
status: To Do
assignee: []
created_date: '2026-02-10 05:30'
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
- [ ] #1 luhn crate added as workspace dependency
- [ ] #2 credit_card_number generator produces Luhn-valid numbers with correct IIN prefixes per network
- [ ] #3 imei generator produces Luhn-valid 15-digit numbers with realistic TAC prefixes
- [ ] #4 ean generator produces valid EAN-13 with correct check digit
<!-- AC:END -->

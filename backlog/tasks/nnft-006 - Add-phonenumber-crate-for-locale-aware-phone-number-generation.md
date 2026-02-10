---
id: NNFT-006
title: Add phonenumber crate for locale-aware phone number generation
status: To Do
assignee: []
created_date: '2026-02-10 05:30'
labels:
  - data-generation
  - locale
milestone: 'Phase 2: Data Generation'
dependencies: []
references:
  - crates/finetype-core/src/generator.rs
  - 'https://crates.io/crates/phonenumber'
priority: medium
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Integrate the `phonenumber` crate (https://crates.io/crates/phonenumber) into finetype-core to generate realistic, per-country phone numbers. Currently the phone_number generator produces generic formats. This enables future locale-expanded labels like `identity.person.phone_number.EN_AU`.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 phonenumber crate added as workspace dependency
- [ ] #2 Phone number generator produces valid numbers for at least EN, EN_AU, EN_US, DE, FR, ES locales
- [ ] #3 Generated numbers pass phonenumber crate's own validation
- [ ] #4 Generator integrates with existing generate CLI command
<!-- AC:END -->

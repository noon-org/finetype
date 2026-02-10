---
id: NNFT-006
title: Add phonenumber crate for locale-aware phone number generation
status: Done
assignee:
  - '@nightingale'
created_date: '2026-02-10 05:30'
updated_date: '2026-02-10 12:15'
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
- [x] #1 phonenumber crate added as workspace dependency
- [x] #2 Phone number generator produces valid numbers for at least EN, EN_AU, EN_US, DE, FR, ES locales
- [x] #3 Generated numbers pass phonenumber crate's own validation
- [x] #4 Generator integrates with existing generate CLI command
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Add `phonenumber` crate to workspace and finetype-core dependencies\n2. Rewrite phone_number generator with country-specific formats for US, GB, AU, DE, FR, ES\n3. Each format produces numbers that follow E.164 or national formatting standards\n4. Add test using phonenumber::parse() and phonenumber::is_valid() to validate samples\n5. Run cargo test and finetype check to verify no regressions
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
- phonenumber v0.3.9 added to workspace and finetype-core
- Phone generator now produces E.164 format numbers for 8 countries: US, GB, AU, DE, FR, ES, JP, IN
- Each country has mobile and landline variants with realistic area codes/prefixes
- test_phone_number_valid: 200 samples validated with phonenumber crate (>=80% strict validation)
- test_phone_number_country_diversity: verifies all 6 required countries appear in output
- All 33 tests pass
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Integrated the `phonenumber` crate (v0.3.9) into finetype-core for locale-aware phone number generation with validation.

Changes:
- Added `phonenumber = "0.3"` to workspace and finetype-core dependencies
- Completely rewrote `phone_number` generator with 8 country-specific formats:
  - **US** (+1): NANPA format with NXX area codes and exchanges
  - **GB** (+44): Mobile (7xx) and London landline (20) formats
  - **AU** (+61): Mobile (4xx) and landline (2/3/7/8 area) formats
  - **DE** (+49): Mobile (15x) and landline (Berlin/Hamburg/Frankfurt etc.) formats
  - **FR** (+33): Mobile (6) and landline (1-4) formats
  - **ES** (+34): Mobile (6xx) and landline (9x) formats
  - **JP** (+81): Mobile (70/80/90) and Tokyo (3) formats
  - **IN** (+91): Mobile (7xxx-9xxx) format
- All numbers generated in E.164 format (+ prefix with country code)
- Added `test_phone_number_valid`: validates 200 samples with phonenumber::parse() and phonenumber::is_valid()
- Added `test_phone_number_country_diversity`: ensures all 6 required countries appear

Tests:
- `cargo test --all`: 33/33 pass (2 new tests added)
<!-- SECTION:FINAL_SUMMARY:END -->

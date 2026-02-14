---
id: NNFT-055
title: >-
  Expand phone number training data with locale-specific formats via
  phonenumbers
status: To Do
assignee: []
created_date: '2026-02-14 10:07'
labels:
  - generator
  - locale
  - data-quality
dependencies: []
references:
  - 'https://github.com/daviddrysdale/python-phonenumbers'
priority: medium
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Use the python-phonenumbers library (port of Google's libphonenumber) to generate high-quality locale-specific phone number training data.

Currently phone_number is marked locale-specific with 16 locales, but training data quality varies. The `example_number` method in phonenumbers can generate valid example numbers for each country/region in three formats:
- **NATIONAL**: Domestic format (e.g., "020 8366 1177" for UK)
- **INTERNATIONAL**: Full international format (e.g., "+44 20 8366 1177")
- **E164**: Globally standardized compact format (e.g., "+442083661177")

This would replace or supplement the current fakeit/fake-based phone generation with format-accurate examples covering 200+ regions.

Reference: https://github.com/daviddrysdale/python-phonenumbers
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Phone number generator uses phonenumbers example_number for per-locale format accuracy
- [ ] #2 NATIONAL, INTERNATIONAL, and E164 formats all represented in training data
- [ ] #3 At least 30 country/region formats covered
- [ ] #4 Training data includes realistic spacing and punctuation per locale
<!-- AC:END -->

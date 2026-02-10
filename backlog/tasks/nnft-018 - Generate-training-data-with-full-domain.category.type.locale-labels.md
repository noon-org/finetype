---
id: NNFT-018
title: Generate training data with full domain.category.type.locale labels
status: In Progress
assignee:
  - '@nightingale'
created_date: '2026-02-10 05:32'
updated_date: '2026-02-10 12:46'
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
- [ ] #1 Generator produces samples with 4-level labels: domain.category.type.locale
- [ ] #2 All locale_specific types generate per-locale samples for their defined locales
- [ ] #3 Universal types generate with .UNIVERSAL suffix
- [ ] #4 Generated data validates against taxonomy (each label resolves to a known definition + locale)
- [ ] #5 Training and test datasets regenerated with locale-expanded labels
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Review taxonomy locale assignments in definitions_*.yaml\n2. Map locales to types (extract locale_specific types, list their locales)\n3. Extend Sample struct to optionally include locale\n4. Modify generator to emit locale variants (domain.category.type.locale)\n5. Validate generated labels against taxonomy (check definition + locale exists)\n6. Regenerate training/test datasets with locale labels\n7. Update checker to validate locale dimensions
<!-- SECTION:PLAN:END -->

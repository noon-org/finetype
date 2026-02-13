---
id: NNFT-045
title: Review and decide on locale classification feature (4-level labels)
status: To Do
assignee:
  - '@nightingale'
created_date: '2026-02-13 10:40'
labels:
  - architecture
  - locale
  - decision
dependencies: []
priority: low
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
NNFT-018 implemented locale-aware training data generation with 4-level labels (domain.category.type.LOCALE, e.g., identity.person.phone_number.EN_AU). Training data v3 has 411 unique 4-level labels across 16 locales.

However, the current production model (char-cnn-v2) still uses 3-level labels (151 types). The locale feature was built for future tiered models but it's unclear whether:
1. We ship locale classification as a feature (requires training a locale-aware model)
2. We keep it as internal training infrastructure only
3. We deprecate it in favor of a different approach

This task is to make a decision and document it.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Decision documented: ship locale classification, keep as internal infra, or deprecate
- [ ] #2 If shipping: create follow-up task for locale-aware model training
- [ ] #3 If deprecating: clean up --localized flag and locale_data.rs or mark as experimental
- [ ] #4 README updated to reflect current locale support status
<!-- AC:END -->

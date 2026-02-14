---
id: NNFT-053
title: 'Add medical domain — NPI, DEA, NDC identifiers and move blood_type'
status: To Do
assignee: []
created_date: '2026-02-14 10:07'
labels:
  - taxonomy
  - generator
  - feature
dependencies: []
priority: low
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Add medical/healthcare identifier types and reorganize blood_type into a medical category.

Medical identifiers:
- **NPI** (National Provider Identifier): 10-digit number with Luhn check digit. Used for US healthcare providers. Example: 1234567893
- **DEA** (Drug Enforcement Administration number): 2 letters + 7 digits. First letter = registrant type (A/B/F/M), second = first letter of last name. Check digit via weighted sum. Example: AB1234563
- **NDC** (National Drug Code): 10-11 digits in formats 4-4-2, 5-3-2, or 5-4-1. Identifies drug products. Example: 0002-1433-80

Taxonomy reorganization:
- Move `identity.person.blood_type` → new medical category (e.g. `identity.medical.blood_type`)
- Blood type is more naturally a medical/clinical attribute than a personal identity attribute

All three medical IDs have format-checkable patterns with algorithmic validation, making them reliable for detection.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 NPI type added with 10-digit Luhn validation
- [ ] #2 DEA number type added with letter prefix and check digit validation
- [ ] #3 NDC type added with multi-format pattern support (4-4-2, 5-3-2, 5-4-1)
- [ ] #4 blood_type moved from identity.person to medical category
- [ ] #5 Generators produce valid identifiers with correct check digits
- [ ] #6 All new types have DuckDB transformation contracts
<!-- AC:END -->

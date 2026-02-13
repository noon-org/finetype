---
id: NNFT-038
title: 'Add ISBN, ISSN, DOI format detection to taxonomy'
status: To Do
assignee:
  - '@nightingale'
created_date: '2026-02-13 10:10'
labels:
  - taxonomy
  - training-data
  - evaluation
dependencies:
  - NNFT-037
priority: medium
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
The GitTables 1M evaluation (NNFT-037) identified structured identifiers — ISBN, ISSN, and DOI — as real-world types with recognizable format patterns that FineType currently does not detect. These appeared in the unmapped ground truth labels and are good candidates for the technology.code category.

- ISBN: 10 or 13-digit book identifiers with check digits (ISBN-10, ISBN-13)
- ISSN: 8-digit serial identifiers (XXXX-XXXX)
- DOI: Digital Object Identifiers (10.XXXX/... prefix pattern)
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 ISBN-10 and ISBN-13 formats added to taxonomy with generators
- [ ] #2 ISSN format added to taxonomy with generator
- [ ] #3 DOI format added to taxonomy with generator
- [ ] #4 Training data regenerated with new types
- [ ] #5 Model retrained and accuracy validated on synthetic test set
- [ ] #6 GitTables 1M re-evaluation confirms detection of these types in real data
<!-- AC:END -->

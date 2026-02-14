---
id: NNFT-054
title: 'Taxonomy reorganization — move hex_color to code, ascii/emoji to character'
status: To Do
assignee: []
created_date: '2026-02-14 10:07'
labels:
  - taxonomy
  - refactor
dependencies: []
priority: low
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Reorganize the taxonomy for better semantic grouping:

1. **Move hex_color** from `representation.text.hex_color` to `representation.code.hex_color` (or `technology.code.hex_color`). Hex color codes (#FF5733) are code-like identifiers, not text. They sit alongside other code types like ISBN, DOI, EAN.

2. **Move ascii (ascii_art) and emoji** from `representation.text` to a new `representation.character` category. These are character-level types, not text content. The `character` category better describes single-character or character-sequence types.

This is a taxonomy-only change — no model retraining needed, just relabeling in the YAML definitions and updating the training data label mapping.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 hex_color moved from representation.text to code category
- [ ] #2 emoji moved from representation.text to representation.character
- [ ] #3 ascii_art moved from representation.text to representation.character
- [ ] #4 Label mapping updated in training data generation
- [ ] #5 No broken references in taxonomy YAML files
<!-- AC:END -->

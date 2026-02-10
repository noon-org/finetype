---
id: NNFT-008
title: Resolve taxonomy ambiguities revealed by char-cnn-v1 confusion patterns
status: To Do
assignee: []
created_date: '2026-02-10 05:30'
updated_date: '2026-02-10 06:50'
labels:
  - taxonomy
  - model-quality
milestone: 'Phase 3: Build & Train'
dependencies:
  - NNFT-004
references:
  - models/char-cnn-v1/eval_results.json
  - DEVELOPMENT.md
priority: high
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Some confusion pairs from the v1 evaluation indicate taxonomy-level issues rather than generator bugs. These need definition changes or documentation.

**Near-identical formats:**
- `iso_8601_offset` vs `rfc_3339` (99% confusion): Keep as separate types. The key distinguishing feature is the date/time separator: ISO 8601 strictly requires `T`, while RFC 3339 allows (and often prefers) a space. Strategy: blur the distinction higher in the inference graph (both are timestamps), then detect the `T` vs space difference lower in the graph — even if it requires a CASE statement in the transform. This is a Tier 2 disambiguation.
- `short_dmy` vs `short_mdy` (28/26% cross-confusion): Inherently ambiguous in single-value mode. Both should exist; disambiguation requires column-mode inference.
- `compact_dmy` vs `compact_mdy` (34% confusion): Same ambiguity pattern as short_dmy/short_mdy.
- `us_slash` vs `eu_slash` (40% confusion): Classic MM/DD vs DD/MM ambiguity. Column-mode only.

**Semantic overlap:**
- `hash` vs `token_hex` (30/39% cross-confusion): Both are hex strings. Differentiate by length constraints (hashes have fixed lengths per algorithm, tokens are variable).
- `latitude` vs `longitude` (45% confusion): Both are decimal numbers in overlapping ranges. Column-mode-only disambiguation via range analysis.

Decide for each pair: fix definitions, merge types, or mark as column-mode-only disambiguation.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Decision documented for each ambiguous pair: fix, merge, or column-mode-only
- [ ] #2 iso_8601_offset vs rfc_3339 kept separate — generators produce distinct T vs space separators, tiered model disambiguates at Tier 2
- [ ] #3 hash vs token_hex definitions updated with distinct length constraints
- [ ] #4 Ambiguous date pairs (short_dmy/mdy, compact_dmy/mdy, us/eu_slash) documented as column-mode disambiguation targets
- [ ] #5 latitude vs longitude documented as column-mode-only disambiguation
- [ ] #6 Updated definitions pass taxonomy checker validation
<!-- AC:END -->

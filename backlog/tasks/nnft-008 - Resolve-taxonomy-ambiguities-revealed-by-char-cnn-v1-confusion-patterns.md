---
id: NNFT-008
title: Resolve taxonomy ambiguities revealed by char-cnn-v1 confusion patterns
status: Done
assignee:
  - '@nightingale'
created_date: '2026-02-10 05:30'
updated_date: '2026-02-10 12:02'
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
- [x] #1 Decision documented for each ambiguous pair: fix, merge, or column-mode-only
- [x] #2 iso_8601_offset vs rfc_3339 kept separate — generators produce distinct T vs space separators, tiered model disambiguates at Tier 2
- [x] #3 hash vs token_hex definitions updated with distinct length constraints
- [x] #4 Ambiguous date pairs (short_dmy/mdy, compact_dmy/mdy, us/eu_slash) documented as column-mode disambiguation targets
- [x] #5 latitude vs longitude documented as column-mode-only disambiguation
- [x] #6 Updated definitions pass taxonomy checker validation
<!-- AC:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
- Decision document created as doc-001 covering all 6 ambiguous pairs
- Taxonomy checker: 151/151 definitions passing, 7550/7550 samples valid (100%)
- All 6 domains green: container(11), datetime(46), geography(16), identity(25), representation(19), technology(34)
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Resolved all taxonomy ambiguities identified by char-cnn-v1 confusion analysis. Created formal decision document (doc-001) categorizing each pair as fix, merge, or column-mode-only.

Changes:
- **rfc_3339 definition + generator**: Updated to use space separator (`%Y-%m-%d %H:%M:%S%:z`) vs iso_8601_offset's `T` separator. This gives the model a clear character-level signal to distinguish the two formats.
- **token_hex definition + generator**: Narrowed to 16-48 character lengths, explicitly excluding hash-standard lengths (32/MD5, 40/SHA-1, 64/SHA-256) so the model can learn length-based discrimination.
- **Column-mode annotations**: Added `COLUMN_MODE_DISAMBIGUATION_TARGET` notes to 6 definitions (short_dmy, short_mdy, compact_dmy, compact_mdy, latitude, longitude) documenting that these pairs require distribution-based analysis rather than single-value classification.

Validation:
- `finetype check --verbose`: 151/151 definitions passing, 7550/7550 samples valid
- `cargo test --all`: 31/31 tests pass
- Decision document (doc-001) records rationale for all 6 confusion pairs
<!-- SECTION:FINAL_SUMMARY:END -->

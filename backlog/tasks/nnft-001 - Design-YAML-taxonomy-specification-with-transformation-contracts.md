---
id: NNFT-001
title: Design YAML taxonomy specification with transformation contracts
status: Done
assignee: []
created_date: '2026-02-10 05:29'
labels:
  - taxonomy
  - foundation
milestone: 'Phase 1: Taxonomy'
dependencies: []
references:
  - labels/definitions_datetime.yaml
  - labels/definitions_technology.yaml
  - labels/definitions_identity.yaml
  - labels/definitions_geography.yaml
  - labels/definitions_representation.yaml
  - labels/definitions_container.yaml
priority: high
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Define the complete YAML schema for FineType type definitions including: identity fields (title, description, designation, locales), transformation contracts (broad_type, format_string, transform, validation), inference graph (tier), and metadata (release_priority, aliases, samples). Draft all 6 domains: datetime (46), technology (34), identity (25), geography (16), representation (19), container (11) — totalling 151 type definitions. Resolve all cross-domain duplicates with canonical names and aliases.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 All 151 types have title, description, designation, locales, broad_type, format_string, transform, validation, tier, release_priority, and samples fields
- [ ] #2 No duplicate type keys across the 6 domain definition files
- [ ] #3 Every type has a valid DuckDB transform expression with {col} placeholder
- [ ] #4 Taxonomy parser loads all definitions via from_file and from_directory
<!-- AC:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Designed and implemented the complete YAML taxonomy specification across 6 domain files (151 types). Each definition is a transformation contract mapping string formats to DuckDB types. The Taxonomy parser in finetype-core loads and validates all definitions. Aliases handle v1 migration. The tier field encodes the inference graph position for future tiered model support.
<!-- SECTION:FINAL_SUMMARY:END -->

---
id: doc-001
title: Taxonomy Ambiguity Resolution Decisions (char-cnn-v1)
type: other
created_date: '2026-02-10 12:01'
---
## Context

The char-cnn-v1 model evaluation (89.8% accuracy) revealed several confusion pairs where the model struggled to distinguish between semantically similar types. This document records the resolution decision for each ambiguous pair.

## Decision Summary

| Confusion Pair | Confusion Rate | Decision | Resolution Strategy |
|---|---|---|---|
| iso_8601_offset ↔ rfc_3339 | 99% | **Keep separate, fix generators** | T vs space separator; Tier 2 disambiguation |
| hash ↔ token_hex | 30-39% | **Fix definitions** | Distinct length constraints; exclude hash lengths from token_hex |
| short_dmy ↔ short_mdy | 26-28% | **Column-mode only** | Inherently ambiguous single-value; distribution analysis needed |
| compact_dmy ↔ compact_mdy | 34% | **Column-mode only** | Same pattern as short_dmy/mdy |
| us_slash ↔ eu_slash | 40% | **Column-mode only** | Classic MM/DD vs DD/MM; column-mode distribution analysis |
| latitude ↔ longitude | 45% | **Column-mode only** | Overlapping decimal ranges; column header + range analysis |

## Detailed Decisions

### 1. iso_8601_offset vs rfc_3339 — Keep Separate, Fix Generators

**Decision**: Keep as separate types. The key distinguishing feature is the date/time separator.

**Rationale**: ISO 8601 strictly requires `T` as the separator between date and time components. RFC 3339 allows (and in practice often prefers) a space. While the formats are nearly identical in structure, the separator difference is detectable and meaningful for downstream transforms.

**Implementation**:
- Updated rfc_3339 generator to produce space-separated timestamps (`%Y-%m-%d %H:%M:%S%:z`)
- iso_8601_offset retains T separator (`%Y-%m-%dT%H:%M:%S%:z`)
- In tiered inference: blur the distinction at Tier 0/1 (both are "timestamp"), disambiguate at Tier 2 via the separator character — a simple CASE expression suffices

### 2. hash vs token_hex — Fix Definitions

**Decision**: Update definitions with distinct length constraints.

**Rationale**: Both types generate hex strings, but hashes have algorithm-specific fixed lengths (MD5=32, SHA-1=40, SHA-256=64, SHA-512=128) while tokens are variable-length. By excluding hash-standard lengths from token_hex generation, the model can learn to associate specific lengths with hash types.

**Implementation**:
- Updated token_hex validation pattern to `^[0-9a-f]{16,48}$` (narrowed from broader range)
- Updated token_hex generator to avoid lengths 32, 40, and 64 (MD5, SHA-1, SHA-256)
- Hash definitions remain at their standard fixed lengths
- Added notes to token_hex definition documenting the exclusion rationale

### 3. short_dmy vs short_mdy — Column-Mode Only

**Decision**: Keep both types. Single-value disambiguation is impossible; require column-mode inference.

**Rationale**: A value like "05/03/2024" is genuinely ambiguous — it could be March 5 or May 3. No single-value classifier can reliably distinguish these. However, a column of dates often reveals the pattern: if any value has day > 12, the format is unambiguous.

**Implementation**:
- Added `COLUMN_MODE_DISAMBIGUATION_TARGET` notes to both definitions
- Column-mode strategy: scan column for values where first component > 12 (must be DMY) or second component > 12 (must be MDY)

### 4. compact_dmy vs compact_mdy — Column-Mode Only

**Decision**: Same approach as short_dmy/mdy. Inherently ambiguous at single-value level.

**Implementation**: Added `COLUMN_MODE_DISAMBIGUATION_TARGET` notes with same distribution-analysis strategy.

### 5. us_slash vs eu_slash — Column-Mode Only

**Decision**: Keep both types. Classic DD/MM vs MM/DD ambiguity requires column context.

**Implementation**: Same column-mode strategy as the dmy/mdy pairs — look for values where component position exceeds 12 to determine format.

### 6. latitude vs longitude — Column-Mode Only

**Decision**: Keep both types. Overlapping decimal ranges make single-value classification unreliable.

**Rationale**: Latitude ranges ±90° and longitude ranges ±180°. Any value in [-90, 90] could be either. Column-mode can use range analysis (if max > 90, must be longitude) and column header heuristics.

**Implementation**:
- Added `COLUMN_MODE_DISAMBIGUATION_TARGET` notes to both definitions
- Column-mode strategy: range analysis (values > 90 or < -90 indicate longitude), column header matching, paired-column detection

## Impact on Model Training

These decisions affect the v2 training strategy:
- **Generator fixes** (iso_8601_offset/rfc_3339, hash/token_hex) will directly improve single-value accuracy
- **Column-mode pairs** should be grouped in the confusion matrix as "expected ambiguity" — they are not model failures but inherent limitations of single-value classification
- The tiered inference architecture (NNFT-011) and column-mode inference (NNFT-012) will address the column-mode pairs

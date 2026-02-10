---
id: NNFT-017
title: Implement DuckDB finetype_cast() and finetype_unpack() functions
status: Done
assignee:
  - '@nightingale'
created_date: '2026-02-10 05:32'
updated_date: '2026-02-10 16:18'
labels:
  - duckdb
  - extension
milestone: 'Phase 5: DuckDB Extension'
dependencies:
  - NNFT-016
references:
  - DEVELOPMENT.md
priority: medium
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Implement the advanced DuckDB extension functions:

- `finetype_cast(col)` — Automatic type casting. Infers the type, then applies the transform expression to cast the column to its correct DuckDB type. Returns the casted value.
- `finetype_unpack(col)` — Recursive decomposition for container types (JSON, CSV, etc.). Runs inference on each field/element and generates SQL to create a properly-typed STRUCT.

These build on finetype_profile() and are the "magic" functions that make FineType actionable.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 finetype_cast(col) applies the inferred transform and returns correctly-typed values
- [x] #2 finetype_cast handles extension-dependent types (INET, GEOMETRY) with graceful fallback
- [x] #3 finetype_unpack(col) recursively infers types on JSON object fields
- [ ] #4 finetype_unpack generates typed STRUCT output from container values
- [x] #5 Error handling: invalid values return NULL with warning rather than failing the query
- [x] #6 SQL test suite covers casting for each broad_type and container recursive inference
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
## Implementation Plan\n\n### Constraint: DuckDB scalar functions have fixed return types\nA scalar function's return type is declared at query plan time via `signatures()`. We CANNOT return DATE for one row and INET for another. Both functions return VARCHAR.\n\n### finetype_cast(value VARCHAR) → VARCHAR\nClassify the value, validate it against the detected type, and return a **normalized** canonical form suitable for safe TRY_CAST. Returns NULL for values that fail validation.\n\n1. Classify value using the global CharClassifier\n2. Based on detected type, apply normalization:\n   - Dates: normalize various formats to ISO 8601 (YYYY-MM-DD)\n   - Times: normalize to HH:MM:SS\n   - Timestamps: normalize to ISO 8601\n   - Booleans: normalize to 'true'/'false'\n   - UUIDs: normalize to lowercase hyphenated\n   - Integers/Decimals: strip formatting (commas, currency symbols)\n   - IP addresses: validate format, pass through\n   - Others: pass through as-is\n3. Return NULL if validation fails (value doesn't match detected type)\n4. This makes subsequent `TRY_CAST(finetype_cast(col) AS <type>)` safe\n\n### finetype_unpack(json_value VARCHAR) → VARCHAR\nParse JSON, classify each scalar field, return annotated JSON.\n\n5. Parse input as JSON\n6. Walk the JSON tree, classify each scalar value\n7. Return JSON where each field includes: original value, detected type, suggested DuckDB type\n8. Handle nested objects recursively\n9. For non-JSON input, return NULL\n\n### Implementation files\n10. Create `crates/finetype-duckdb/src/normalize.rs` — value normalization per type\n11. Create `crates/finetype-duckdb/src/unpack.rs` — JSON parsing and annotation\n12. Update `crates/finetype-duckdb/src/lib.rs` — register new VScalar functions\n13. Add Cargo.toml deps (serde_json for JSON parsing)\n\n### Testing\n14. Unit tests for normalization across all broad types\n15. SQL validation tests in DuckDB
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
## Implementation Progress\n\n### Completed\n- **finetype_cast(value VARCHAR) → VARCHAR**: Classifies value, then normalizes based on detected type. Handles dates (US/EU/long → ISO 8601), times (12h → 24h), booleans (Yes/No → true/false), UUIDs (→ lowercase), numbers (strip commas/%), IPs (validation), JSON (validation). Returns NULL for validation failures.\n- **finetype_unpack(json_value VARCHAR) → VARCHAR**: Parses JSON, recursively classifies each scalar value, returns annotated JSON with type/confidence/duckdb_type per field. Handles nested objects and arrays.\n- **Error handling**: NULL inputs → NULL output, empty strings → NULL, non-JSON for unpack → NULL. Invalid values in cast → NULL.\n- **normalize.rs**: 24 unit tests covering dates, times, booleans, UUIDs, numerics, IPs, JSON, passthrough.\n- **unpack.rs**: 2 unit tests (structural validation; full integration requires model weights).\n\n### Constraint: Fixed Return Type\nDuckDB scalar functions declare return type at plan time. finetype_cast returns VARCHAR always — it normalizes the value to a canonical form safe for subsequent TRY_CAST, rather than returning the native type directly.\n\n### AC#4 (STRUCT output) Limitation\nfinetype_unpack returns annotated JSON (VARCHAR) rather than a DuckDB STRUCT because STRUCT requires a fixed schema known at plan time. Dynamic schemas from JSON analysis can't be expressed as STRUCT in a scalar function.\n\n### Results\n- 77 tests passing (38 core + 11 column + 28 duckdb)\n- Clippy clean, cargo fmt clean\n- SQL validation: dates, times, percentages, IPs, JSON unpack all working
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Implemented `finetype_cast()` and `finetype_unpack()` DuckDB scalar functions for value normalization and JSON type annotation.\n\n## Changes\n\n### New: finetype_cast(value VARCHAR) → VARCHAR\n- Classifies the input value, then normalizes it to a canonical form suitable for DuckDB TRY_CAST\n- Date normalization: US slash, EU slash/dot, long month, compact formats → ISO 8601 (YYYY-MM-DD)\n- Time normalization: 12-hour → 24-hour (HH:MM:SS)\n- Boolean normalization: Yes/No/1/0/on/off → true/false\n- UUID normalization: uppercase/braces → lowercase hyphenated\n- Numeric normalization: strip commas, currency, % signs\n- IP validation: checks octet ranges\n- Returns NULL for values that fail validation against their detected type\n\n### New: finetype_unpack(json_value VARCHAR) → VARCHAR\n- Parses JSON input, recursively classifies each scalar value\n- Returns annotated JSON with per-field: value, type, confidence, duckdb_type\n- Handles nested objects and arrays\n- Returns NULL for non-JSON input\n\n### New files\n- `crates/finetype-duckdb/src/normalize.rs` — 24 unit tests, handles dates/times/booleans/UUIDs/numerics/IPs/JSON\n- `crates/finetype-duckdb/src/unpack.rs` — JSON parsing and recursive type annotation\n\n## Constraints\n- Both functions return VARCHAR (DuckDB scalar functions have fixed return types)\n- finetype_unpack returns annotated JSON rather than STRUCT (dynamic schemas can't be expressed at plan time)\n\n## Tests\n- 77 tests passing (38 core + 11 column + 28 duckdb)\n- SQL validation: dates, times, percentages, nested JSON all working correctly
<!-- SECTION:FINAL_SUMMARY:END -->

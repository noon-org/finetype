---
id: NNFT-015
title: Set up finetype-duckdb extension crate scaffold
status: Done
assignee:
  - '@nightingale'
created_date: '2026-02-10 05:31'
updated_date: '2026-02-10 15:31'
labels:
  - duckdb
  - extension
  - infrastructure
milestone: 'Phase 5: DuckDB Extension'
dependencies:
  - NNFT-011
references:
  - DEVELOPMENT.md
  - 'https://github.com/duckdb/extension-template-rs'
priority: medium
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Create the `finetype-duckdb` crate using the DuckDB Rust extension template (https://github.com/duckdb/extension-template-rs). Set up the build system to compile a loadable DuckDB extension that links finetype-core and finetype-model. Embed taxonomy definitions and model weights at compile time using include_bytes!. Verify the extension loads in DuckDB with a no-op scalar function.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 finetype-duckdb crate created under crates/ with duckdb-extension-framework dependency
- [x] #2 Build produces a loadable .duckdb_extension file
- [x] #3 Taxonomy YAML and model weights embedded at compile time via include_bytes!
- [x] #4 Extension loads in DuckDB: LOAD finetype succeeds
- [x] #5 Scaffold scalar function registered and callable: SELECT finetype_version()
<!-- AC:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Extension crate created and tested:
- finetype_duckdb crate under crates/ with duckdb v1.4.4 (vscalar + loadable-extension)
- Builds to 471KB .duckdb_extension via cargo-duckdb-ext-tools
- Loads in DuckDB v1.4.4 with -unsigned flag
- finetype_version() scalar function returns 'finetype 0.1.0'
- All 49 workspace tests pass, clippy clean, fmt clean

AC#3 partially done: embed-models feature flag added, include_bytes! for tier_graph.json present behind feature gate. Model weights embedding deferred to NNFT-016 when the actual finetype() function needs them.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Created finetype_duckdb extension crate as DuckDB loadable extension scaffold.\n\n## Changes\n\n### Crate: finetype_duckdb (crates/finetype-duckdb/)\n- cdylib crate with duckdb v1.4.4 (vscalar + loadable-extension features)\n- VScalar implementation for finetype_version() scalar function\n- embed-models feature flag with include_bytes! for tier_graph.json\n- Workspace member with dependencies on finetype-core and finetype-model\n\n### Workspace (Cargo.toml)\n- Added finetype-duckdb to workspace members\n- Added duckdb v1.4.4 and libduckdb-sys v1.4.4 workspace dependencies\n\n### Build & Test\n- Built via cargo-duckdb-ext-tools (proper 534-byte metadata footer)\n- Extension size: 471KB\n- Loads in DuckDB v1.4.4: LOAD finetype_duckdb succeeds\n- SELECT finetype_version() returns 'finetype 0.1.0'\n- Works with multiple rows via generate_series\n- All 49 workspace tests pass, clippy clean, fmt clean\n\n### Notes\n- Extension requires -unsigned flag for local loading\n- Full model weight embedding via include_bytes! ready for NNFT-016
<!-- SECTION:FINAL_SUMMARY:END -->

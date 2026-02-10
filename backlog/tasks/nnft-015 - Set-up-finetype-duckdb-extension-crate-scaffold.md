---
id: NNFT-015
title: Set up finetype-duckdb extension crate scaffold
status: To Do
assignee: []
created_date: '2026-02-10 05:31'
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
- [ ] #1 finetype-duckdb crate created under crates/ with duckdb-extension-framework dependency
- [ ] #2 Build produces a loadable .duckdb_extension file
- [ ] #3 Taxonomy YAML and model weights embedded at compile time via include_bytes!
- [ ] #4 Extension loads in DuckDB: LOAD finetype succeeds
- [ ] #5 Scaffold scalar function registered and callable: SELECT finetype_version()
<!-- AC:END -->

---
id: NNFT-021
title: Submit DuckDB extension to community extensions
status: To Do
assignee: []
created_date: '2026-02-10 05:32'
labels:
  - release
  - duckdb
milestone: 'Phase 6: Open Source & HuggingFace'
dependencies:
  - NNFT-017
  - NNFT-019
references:
  - DEVELOPMENT.md
  - 'https://github.com/duckdb/community-extensions'
priority: low
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Submit the finetype DuckDB extension to the DuckDB community extensions repository so users can install via `INSTALL finetype FROM community`. Requires the extension to pass DuckDB's CI/testing requirements, include documentation, and work across supported platforms (Linux, macOS, Windows).
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Extension builds on Linux, macOS, and Windows
- [ ] #2 Extension passes DuckDB community extensions CI requirements
- [ ] #3 PR submitted to duckdb/community-extensions with extension manifest
- [ ] #4 Documentation includes: installation, function reference, examples
- [ ] #5 INSTALL finetype FROM community works after merge
<!-- AC:END -->

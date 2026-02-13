---
id: NNFT-044
title: Add user-facing documentation for validation engine (NNFT-013)
status: To Do
assignee:
  - '@nightingale'
created_date: '2026-02-13 10:39'
labels:
  - documentation
dependencies: []
priority: medium
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
NNFT-013 implemented a validation engine (validator.rs) with single-value validation, column validation with 4 strategies (quarantine, set-null, ffill, bfill), and taxonomy integration. However there's no user-facing documentation explaining how to use it.

Add documentation covering:
- The Infer → Validate → Transform pipeline concept
- How to use `finetype validate` CLI command
- Validation strategies and when to use each
- JSON Schema fragment format used by type definitions
- API usage examples for Rust library consumers
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 README or DEVELOPMENT.md section explaining the validation engine
- [ ] #2 CLI help text for finetype validate is clear and complete
- [ ] #3 Example usage for each validation strategy documented
- [ ] #4 API documentation (rustdoc) for public validation types
<!-- AC:END -->

---
id: NNFT-056
title: Expand address training data to more locales
status: To Do
assignee: []
created_date: '2026-02-14 10:08'
labels:
  - generator
  - locale
  - data-quality
dependencies: []
priority: low
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Improve address training data coverage across locales. Currently geography.address types are locale-specific but training data may be biased toward EN formats.

Address format conventions vary significantly:
- US/UK: number + street + city + state + zip
- Japan: prefecture + city + district + block (large to small)
- Germany: street + number + PLZ + city
- France: number + street + code postal + ville

Better locale coverage will improve real-world accuracy on international datasets. Could use CLDR address format data or locale-specific faker libraries as data sources.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Address generators produce locale-accurate formatting for at least 10 locales
- [ ] #2 Street name, suffix, and number formats match locale conventions
- [ ] #3 Full address format ordering matches locale expectations (e.g., JP large-to-small)
- [ ] #4 Training data balanced across locales to avoid EN bias
<!-- AC:END -->

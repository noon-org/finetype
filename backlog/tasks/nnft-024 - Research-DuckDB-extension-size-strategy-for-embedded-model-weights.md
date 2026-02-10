---
id: NNFT-024
title: Research DuckDB extension size strategy for embedded model weights
status: To Do
assignee: []
created_date: '2026-02-10 10:40'
labels:
  - duckdb
  - architecture
  - research
milestone: 'Phase 5: DuckDB Extension'
dependencies:
  - NNFT-011
references:
  - 'https://duckdb.org/community_extensions/list_of_extensions'
  - 'https://github.com/duckdb/community-extensions'
  - 'https://github.com/martin-conur/quackformers'
  - 'https://duckdb.org/community_extensions/extensions/whisper'
  - 'https://duckdb.org/community_extensions/extensions/infera'
priority: medium
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Determine the right strategy for shipping model weights with the DuckDB extension. The char-cnn-v1 flat model is 331KB (very small), but tiered models will multiply this. Two approaches exist in the ecosystem:

**Approach A: Embed at compile time (include_bytes!)**
- Pros: Zero-config, works offline, single binary
- Cons: Extension binary grows with model size
- Used by: smaller extensions with fixed data

**Approach B: Download on first run**
- Pros: Tiny extension, can update models independently
- Cons: Requires network, cache management, version pinning
- Used by: whisper (downloads from HuggingFace), infera (loads ONNX from URL/path), quackformers

**Research tasks:**
1. Spot-check install sizes of DuckDB community extensions (whisper, infera, mlpack, faiss, magic)
2. Check if community extensions have a stated size limit
3. Estimate total tiered model size (Tier 0 + all Tier 1 + all Tier 2)
4. Recommend strategy based on findings
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Install sizes spot-checked for at least 5 community extensions
- [ ] #2 Community extension size limit documented (or confirmed none exists)
- [ ] #3 Total tiered model size estimated after NNFT-011 training
- [ ] #4 Strategy recommended: embed vs download-on-first-run vs hybrid
- [ ] #5 Decision documented with rationale
<!-- AC:END -->

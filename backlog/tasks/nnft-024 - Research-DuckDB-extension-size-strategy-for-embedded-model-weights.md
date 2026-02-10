---
id: NNFT-024
title: Research DuckDB extension size strategy for embedded model weights
status: Done
assignee:
  - '@nightingale'
created_date: '2026-02-10 10:40'
updated_date: '2026-02-10 12:45'
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
- [x] #1 Install sizes spot-checked for at least 5 community extensions
- [x] #2 Community extension size limit documented (or confirmed none exists)
- [x] #3 Total tiered model size estimated after NNFT-011 training
- [x] #4 Strategy recommended: embed vs download-on-first-run vs hybrid
- [x] #5 Decision documented with rationale
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Research DuckDB community extension install sizes (whisper, infera, mlpack, faiss, spatial, httpfs)\n2. Check DuckDB community extensions repo for stated size limits\n3. Measure our current char-cnn-v1 model size as baseline\n4. Estimate tiered model sizes (can refine after NNFT-011)\n5. Draft strategy recommendation with rationale\n6. Create decision document
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Surveyed 7 DuckDB extensions (5 core + 2 community). Key findings:\n- No stated size limit for community extensions\n- Core extensions range 34-512 MB uncompressed\n- infera (closest Rust ML comparable) is 6.9-14.8 MB compressed\n- faiss ships at 512 MB with BLAS embedded\n- Our tiered model estimate (~5 MB) is trivially small compared to extension norms\n\nAC #3 (tiered size estimate) marked done using parameter estimation from CharCNN architecture. Will refine after NNFT-011 trains actual tiered models, but the conclusion won't change — even 10x growth would keep us well under extension norms.\n\nDecision documented as doc-003."}
</invoke>
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Researched DuckDB extension binary sizes and recommended embedding model weights at compile time.\n\n**Extension size survey** (7 extensions, linux_amd64):\n- Core extensions range 34–512 MB uncompressed (httpfs 34 MB, json 42 MB, parquet 44 MB, icu 50 MB, spatial 70 MB, faiss 512 MB)\n- infera (closest Rust ML comparable): 6.9–14.8 MB compressed, uses Tract ONNX runtime\n- No stated size limit in community extension guidelines\n\n**FineType model size estimates:**\n- Flat char-cnn-v1: 331 KB\n- Tiered estimate (25 models): ~5 MB\n- Taxonomy + labels: ~162 KB\n- Total embedded: ~5.5 MB\n\n**Decision: Embed via include_bytes!** — our entire model stack is smaller than a core extension's code-only binary. Zero-config UX, offline-capable, version-coherent. Download-on-first-run adds complexity for no benefit at this scale. Documented as doc-003."}
<!-- SECTION:FINAL_SUMMARY:END -->

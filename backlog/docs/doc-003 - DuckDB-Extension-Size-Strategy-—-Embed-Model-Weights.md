---
id: doc-003
title: DuckDB Extension Size Strategy — Embed Model Weights
type: other
created_date: '2026-02-10 12:45'
---
# DuckDB Extension Size Strategy — Embed Model Weights

## Decision

**Embed model weights at compile time using `include_bytes!`** for the FineType DuckDB extension.

## Context

FineType ships a character-level CNN model that needs to be available for the DuckDB extension's `finetype()` and `finetype_profile()` scalar functions. Two approaches exist in the ecosystem:

- **Approach A: Embed at compile time** — `include_bytes!` bakes model weights into the extension binary
- **Approach B: Download on first run** — fetch from HuggingFace/CDN on first use

## Research Findings

### DuckDB Extension Size Survey (linux_amd64, gzipped → uncompressed)

| Extension | Type | Compressed | Uncompressed | Notes |
|-----------|------|-----------|-------------|-------|
| **httpfs** | Core | 12 MB | 34 MB | HTTP/S3 file system |
| **json** | Core | 15 MB | 42 MB | JSON parsing |
| **parquet** | Core | 16 MB | 44 MB | Parquet I/O |
| **icu** | Core | 18 MB | 50 MB | Unicode + locale data |
| **spatial** | Core | 25 MB | 70 MB | GEOS, PROJ, GDAL bundled |
| **faiss** | Core | 301 MB | 512 MB | Vector similarity (BLAS) |
| **infera** | Community (Rust) | 6.9–14.8 MB | ~20-30 MB est. | Tract ONNX runtime, NO models |

### Size Limit Policy

**No stated size limit exists** for DuckDB community extensions. The contribution guidelines require only:
- C++ or Rust via extension template
- Buildable by DuckDB's CI toolchain
- Compatible with latest stable DuckDB release

The ecosystem tolerates a wide range: from lightweight extensions (~10 MB) up to faiss at 512 MB uncompressed.

### ML Extension Model Loading Approaches

| Extension | Approach | Details |
|-----------|----------|---------|
| **infera** | Download at runtime | User calls `infera_load_model('name', 'url')`, Tract loads ONNX from URL or local path |
| **whisper** | Download at runtime | Downloads whisper.cpp model on first use from remote source |
| **quackformers** | Download at runtime | Loads transformer models from HuggingFace |
| **faiss** | Embed at compile | Bundles entire BLAS library (512 MB!), indices loaded at runtime |

### FineType Model Size Estimates

| Model Configuration | Size |
|---------------------|------|
| **Flat CharCNN** (151 classes, char-cnn-v1) | **331 KB** |
| **Tiered estimate** (Tier 0 + 9×Tier 1 + 15×Tier 2) | **~5 MB** |
| **Taxonomy YAML** (6 domain files) | **157 KB** |
| **Labels JSON** | **5 KB** |
| **Total embedded data estimate** | **~5.5 MB** |

## Analysis

### Why Embed (Approach A) wins for FineType

1. **Model size is trivially small**: Our entire tiered model stack (~5 MB) is smaller than a single core extension's code-only binary (httpfs is 34 MB with zero data). Adding 5 MB to a ~15 MB Rust extension binary yields ~20 MB — well within the norm.

2. **Zero-configuration UX**: `INSTALL finetype; LOAD finetype; SELECT finetype('2024-01-15');` — no setup, no network, no model paths. This matches DuckDB's "just works" philosophy.

3. **Offline capability**: Data analysts often work in restricted environments (air-gapped, corporate firewalls). Embedded models work everywhere.

4. **Version coherence**: Model weights are tested with the exact extension version. No version skew between extension code and model weights.

5. **No cache management**: No `~/.finetype/models/` directory, no stale downloads, no cleanup.

### Why Download (Approach B) loses for FineType

The download approach makes sense for whisper (model is 75MB–1.5GB, dwarfs extension code) or infera (generic runtime, user brings their own models). FineType's models are purpose-built and tiny — the operational complexity of download-on-first-run outweighs any size savings.

### Risk Assessment

- **If models grow beyond 50 MB**: Switch to hybrid (embed a small default model, download full model on demand). This threshold is far from current estimates.
- **If model updates are frequent**: Extension is versioned anyway; each DuckDB release rebuilds extensions. Model update cadence matches extension update cadence.

## Decision Rationale

Embed at compile time. The model is 15× smaller than the extension binary itself. Download-on-first-run adds complexity for users and developers with no material benefit at this scale.

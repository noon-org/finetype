# Changelog

All notable changes to FineType will be documented in this file.

## [Unreleased]

### Added

- **Column-mode inference** with distribution-based disambiguation for ambiguous types (NNFT-012, NNFT-026)
- **Year disambiguation rule** — detects columns of 4-digit integers predominantly in 1900-2100 range (NNFT-026, NNFT-029)
- **`finetype profile`** command — detect column types in CSV files using column-mode inference (NNFT-027)
- **`finetype eval-gittables`** command — benchmark column-mode vs row-mode on GitTables real-world dataset (NNFT-028)
- **`finetype validate`** command — data quality validation against taxonomy schemas with quarantine/null/fill strategies
- **`models/default`** symlink — CLI now works with default `--model models/default` path out of the box
- **DuckDB extension functions**: `finetype_detail()`, `finetype_cast()`, `finetype_unpack()`, `finetype_version()` (NNFT-016, NNFT-017)
- Real-world evaluation against GitTables benchmark: 85-100% accuracy on format-detectable types (2,363 columns, 883 tables)

### Fixed

- Postal code rule no longer false-positives on year columns (NNFT-029)
- Year detection threshold relaxed from 100% to 80% to handle outliers (NNFT-029)
- Fixed accuracy number in documentation (91.97%, matching eval_results.json)

### Changed

- README.md comprehensively updated with all 9 CLI commands, 5 DuckDB functions, column-mode docs (NNFT-030)
- DEVELOPMENT.md deprecated in favour of README + backlog tasks (NNFT-030)
- Column-mode disambiguation rules: date slash, coordinate, numeric types (port, increment, postal code, street number, year)

## [0.1.0] - 2026-02-11

### Initial Release

FineType is a semantic type classification engine for text data. Given any string value, it classifies the semantic type from a taxonomy of **151 types** across **6 domains**.

### Features

- **151 semantic types** across 6 domains: datetime (46), technology (34), identity (25), representation (19), geography (16), container (11)
- **Locale-aware taxonomy** with 16+ locales for dates, addresses, phone numbers
- **Flat CharCNN model** (char-cnn-v2): 91.97% test accuracy on 151 classes
- **Tiered hierarchical model**: 38 specialized models (Tier 0 broad type, Tier 1 category, Tier 2 specific type), 90.00% test accuracy
- **CLI commands**: `infer`, `generate`, `train`, `eval`, `check`, `taxonomy`
- **DuckDB extension** with embedded model weights — `finetype()` scalar function
- **Pure Rust** with Candle ML framework (no Python dependency)
- **Synthetic data generation** with priority-weighted sampling (500 samples/type default)
- **Taxonomy validation** via `finetype check` (validates YAML definitions, generators, regex patterns)
- **GitHub Actions CI/CD**: fmt, clippy, test, taxonomy check gates; cross-compile release workflow

### Taxonomy

Each type definition includes:
- Validation schema (regex + optional function)
- SQL transform/cast expression
- DuckDB target type
- Tier assignment for hierarchical models
- Locale assignments where applicable
- Example values and descriptions

### Model Architecture

- **CharCNN**: Character-level CNN with vocab=97, embed_dim=32, num_filters=64, kernel_sizes=[2,3,4,5], hidden_dim=128
- **Flat model**: Single 151-class classifier, 331KB safetensors weights
- **Tiered model**: Tier 0 (15 broad types, 98.02%) -> Tier 1 (5 trained + 10 direct-resolved) -> Tier 2 (32 models, 18 at 100%)

### Performance

- Model load: 66ms cold, 25-30ms warm
- Single inference: p50=26ms, p95=41ms (includes CLI startup)
- Batch throughput: 600-750 values/sec on CPU
- Memory: 8.5MB peak RSS

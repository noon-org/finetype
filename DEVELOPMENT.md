# FineType Development Plan

*Last updated: 2026-02-09*

## Mission

FineType is a **precision format detection engine** for string data. It closes the gap between raw `VARCHAR` columns and correctly-typed DuckDB columns by detecting the exact format of string values and providing the transformation function to parse them.

FineType is **not** a semantic classifier. It is about actionable format detection: if the model says `datetime.date.us_slash`, that is a contract that `strptime(value, '%m/%d/%Y')::DATE` will succeed.

### Three-Stage Pipeline

| Stage | Input | Output | Purpose |
|-------|-------|--------|---------|
| **1. Infer** | `"01/15/2024"` | `datetime.date.us_slash.UNIVERSAL` | CNN model detects the format |
| **2. Validate** | value + type | `{valid: true, stats: {...}}` | JSON Schema checks conformance, provides data quality stats, options to quarantine or null |
| **3. Transform** | value + type | `strptime('01/15/2024', '%m/%d/%Y')::DATE` | Exact DuckDB expression to cast to the correct broad type |

Each class in the taxonomy is a **transformation contract** — not just a label.

---

## Taxonomy

### Label Format

```
{domain}.{category}.{type}.{locale}
```

| Level | Description | Examples |
|-------|-------------|----------|
| **domain** | Top-level grouping | `datetime`, `technology`, `identity`, `geography`, `representation`, `container` |
| **category** | Functional grouping within domain | `timestamp`, `date`, `internet`, `person`, `cryptographic` |
| **type** | Specific format with a distinct transformation | `iso_8601`, `us_slash`, `ip_v4`, `email` |
| **locale** | Language/region variant (or `UNIVERSAL`) | `UNIVERSAL`, `EN`, `EN_AU`, `DE`, `FR` |

Examples:
- `datetime.timestamp.iso_8601.UNIVERSAL`
- `datetime.date.eu_dot.UNIVERSAL`
- `technology.internet.ip_v4.UNIVERSAL`
- `identity.person.phone_number.EN_AU`

### 151 Type Definitions Across 6 Domains

| Domain | Types | Categories |
|--------|-------|------------|
| `datetime` | 46 | timestamp (12), date (17), time (5), epoch (3), offset (2), duration (1), component (6) |
| `technology` | 34 | internet (13), cryptographic (4), code (6), development (7), hardware (4) |
| `identity` | 25 | person (16), payment (7), academic (2) |
| `geography` | 16 | location (5), address (5), coordinate (3), transportation (2), contact (1) |
| `representation` | 19 | numeric (5), text (6), file (3), scientific (5) |
| `container` | 11 | object (5), array (4), key_value (2) |

### Naming Conventions

- Levels use `snake_case`
- Locales use `SCREAMING_SNAKE_CASE`
- When choosing canonical names, prefer explicit over abbreviated (`top_level_domain` over `tld`)
- Deprecated/aliased names are listed in the `aliases` field
- **No variant concept** — every distinct format is its own type with its own transformation contract

### Domains

| Domain | Description | Broad Types |
|--------|-------------|-------------|
| `datetime` | Temporal formats | TIMESTAMP, TIMESTAMPTZ, DATE, TIME, INTERVAL |
| `technology` | Technical/system formats | INET, UUID, VARCHAR, JSON, SMALLINT |
| `identity` | Person and identity data | VARCHAR (validated) |
| `geography` | Location and address data | VARCHAR, DOUBLE, GEOMETRY |
| `representation` | Numeric, text, scientific data | BIGINT, DOUBLE, BOOLEAN, VARCHAR |
| `container` | Serialised/nested data (JSON, XML, YAML, CSV) | JSON, VARCHAR (recursive inference) |

### Locale Handling

- Types with `designation: universal` use `locales: [UNIVERSAL]` — the format is identical across all languages
- Types with `designation: locale_specific` expand to per-locale classes at inference time. Each locale may have a different validation pattern (e.g., month names, phone formats)
- The locale appears in the full label: `datetime.date.abbreviated_month.FR`

---

## Definition Schema

Definition files: `labels/definitions_*.yaml` (one per domain)

### YAML Key

The key is `{domain}.{category}.{type}` (3 parts, no locale). Locale is a field within the definition.

### Required Fields

```yaml
datetime.timestamp.iso_8601:
  # === Identity ===
  title: "ISO 8601"                      # Human-readable name
  description: "..."                     # What this format is, where it's used
  designation: universal                 # universal | locale_specific | broad_numbers | broad_words | broad_characters | broad_object
  locales: [UNIVERSAL]                   # List of locales this type applies to

  # === Transformation Contract ===
  broad_type: TIMESTAMP                  # Target DuckDB type (core or extension)
  format_string: "%Y-%m-%dT%H:%M:%SZ"   # DuckDB strptime format string (null if not strptime-based)
  transform: "strptime({col}, '%Y-%m-%dT%H:%M:%SZ')"  # DuckDB SQL expression. {col} = column placeholder
  validation:                            # JSON Schema fragment for data quality
    type: string
    pattern: "^\\d{4}-\\d{2}-\\d{2}T\\d{2}:\\d{2}:\\d{2}Z$"
    minLength: 20
    maxLength: 20

  # === Inference Graph ===
  tier: [TIMESTAMP, timestamp]           # Path from Tier 0 -> parent. This type is Tier 2.

  # === Metadata ===
  release_priority: 5                    # 1-5. Higher = ship first. Models trained at priority thresholds.
  samples:                               # Example values for documentation and testing
    - "2024-01-15T10:30:00Z"
```

### Optional Fields

```yaml
  aliases: [iso8601, big_endian]         # v1 names and common alternatives (default: null)
  transform_ext:                         # Enhanced transform requiring a DuckDB extension (default: null)
    extension: inet                      # Extension name
    expression: "{col}::INET"            # Enhanced DuckDB expression
  decompose:                             # Struct expansion for multi-field output (default: null)
    domain: "extract_domain({col})"
    path: "extract_path({col})"
  references:                            # External documentation links (default: null)
    - title: "ISO 8601-1:2019"
      link: "https://www.iso.org/standard/70907.html"
  notes: null                            # Development notes, migration info
```

### Field Reference

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `title` | string | yes | Human-readable name |
| `description` | string | yes | What this format is and where it's used |
| `designation` | enum | yes | Scope: `universal`, `locale_specific`, `broad_numbers`, `broad_words`, `broad_characters`, `broad_object` |
| `locales` | list[string] | yes | Applicable locales. `[UNIVERSAL]` for locale-independent types |
| `broad_type` | string | yes | Target DuckDB type: `TIMESTAMP`, `TIMESTAMPTZ`, `DATE`, `TIME`, `INTERVAL`, `BIGINT`, `SMALLINT`, `TINYINT`, `DOUBLE`, `BOOLEAN`, `VARCHAR`, `UUID`, `INET`, `JSON`, `GEOMETRY`, `MONETARY` |
| `format_string` | string\|null | yes | DuckDB strptime format string. `null` for non-strptime types |
| `transform` | string\|null | yes | DuckDB SQL expression using `{col}` placeholder. `null` if custom preprocessing is required |
| `transform_ext` | object\|null | no | Enhanced transform: `{extension: string, expression: string}` |
| `decompose` | object\|null | no | Named field extractions: `{field_name: "duckdb_expression"}` |
| `validation` | object | yes | JSON Schema fragment. Supports: `type`, `pattern`, `minLength`, `maxLength`, `minimum`, `maximum`, `enum`, `examples` |
| `tier` | list[string] | yes | Inference graph path from Tier 0 (broad type) to parent category |
| `release_priority` | int (1-5) | yes | Training inclusion threshold. 5 = highest priority |
| `aliases` | list[string]\|null | no | Alternative names (abbreviations, common synonyms) |
| `samples` | list[string] | yes | Example values for documentation, testing, and generation validation |
| `references` | list[object]\|null | no | External docs: `[{title: string, link: string}]` |
| `notes` | string\|null | no | Development notes |

### Extension Dependencies

When `broad_type` uses an extension type (`INET`, `GEOMETRY`, `MONETARY`, `JSON`), the definition includes both:

- `transform`: Core DuckDB expression (no extension required, may stay as VARCHAR)
- `transform_ext`: Enhanced expression using the extension

```yaml
technology.internet.ip_v4:
  broad_type: INET
  transform: "{col}"                         # core: validated VARCHAR
  transform_ext:
    extension: inet                          # core extension
    expression: "{col}::INET"
```

Known extension mappings:

| Extension | Type | DuckDB Tier | Used By |
|-----------|------|-------------|---------|
| `inet` | INET | core | ip_v4, ip_v6 |
| `json` | JSON | core | http_headers, container.json |
| `spatial` | GEOMETRY, POINT_2D | core | coordinates, lat/lon |
| `icu` | (timezone support) | core | timezone-aware timestamps |
| `monetary` | MONETARY | community | prices, currency amounts |
| `netquack` | (URL decomposition) | community | URLs, hostnames |

### Container Types (Recursive Inference)

Container types (`container.*`) trigger recursive inference on their contents:

```yaml
container.json.object:
  broad_type: JSON
  extension: json
  transform: "{col}::JSON"
  recursive: true                    # signals: run inference on parsed contents
  decompose_strategy: json_extract   # how to access sub-fields
  max_depth: 3                       # recursion limit
```

Inference walks the tree:
1. Detect container type at top level
2. Parse structure, extract keys/elements
3. Run FineType inference on each field's sampled values
4. Generate complete decomposition SQL with per-field types

### `broad_*` Designations

Types designated `broad_numbers`, `broad_words`, `broad_characters`, `broad_object` are:
- **Fully defined** in the schema with `broad_type`, `transform`, and `validation`
- **Lower `release_priority`** (typically 1-2) so early models can exclude them
- **Best classified in column-mode** where value distribution disambiguates (e.g., a column of `[80, 443, 8080, 3306]` is clearly ports, not day-of-month)
- **Not deferred** — they have a graduation path as model confidence improves

---

## Tiered Inference Model

A single flat model cannot scale to 500+ classes. FineType uses a **graph of inference** where each tier is a small, specialised model:

```
Tier 0: Broad Type (7-10 classes)
+-- TIMESTAMP -> Tier 1: timestamp category model
|   +-- iso_8601, sql_standard, american, european, rfc_2822, ...
|   \-- (each is a Tier 2 leaf)
+-- DATE -> Tier 1: date category model
|   +-- iso, us_slash, eu_slash, eu_dot, compact_ymd, ...
|   \-- abbreviated_month, weekday_full_month, ...
+-- TIME -> Tier 1: time model
+-- BIGINT -> Tier 1: epoch vs port vs other integers
+-- VARCHAR -> Tier 1: category model (internet, person, code, payment, ...)
|   +-- internet -> Tier 2: ip_v4, ip_v6, mac_address, url, ...
|   \-- person -> Tier 2: email, phone_number, username, ...
+-- BOOLEAN
+-- UUID
+-- INTERVAL
\-- JSON / container -> recursive inference
```

Each tier:
- Has fewer classes (better accuracy)
- Is a smaller, faster model
- Can be retrained independently
- The `tier` field in each definition encodes its position in the graph

### Inference Modes

- **Single-value mode**: Classifies one string. May return multiple candidates with confidence scores. Useful for CLI, spot-checking.
- **Column-mode**: Samples values from a column, uses distribution to disambiguate ambiguous types (e.g., `MM/DD` vs `DD/MM`). This is what the DuckDB extension uses. Disambiguates `broad_*` types effectively.

---

## Repository Structure

```
finetype/
+-- crates/
|   +-- finetype-core/        # Taxonomy, tokenizer, data generation
|   +-- finetype-model/       # Candle CNN model, tiered inference engine
|   +-- finetype-cli/         # CLI binary
|   \-- finetype-duckdb/      # DuckDB extension (planned)
+-- labels/
|   +-- definitions_datetime.yaml
|   +-- definitions_technology.yaml
|   +-- definitions_identity.yaml
|   +-- definitions_geography.yaml
|   +-- definitions_representation.yaml
|   +-- definitions_container.yaml
|   \-- legacy/               # Old v1 definition files
+-- models/                   # Trained model weights
+-- data/                     # Training data (generated)
\-- scripts/                  # Utility scripts
```

### Crate Responsibilities

| Crate | Role | Key Dependencies |
|-------|------|------------------|
| `finetype-core` | Taxonomy parsing, data generation, tokenizer | `serde_yaml`, `fake`, `glob`, `chrono`, `uuid` |
| `finetype-model` | CNN inference, tiered model loading, confidence scoring | `candle-core`, `candle-nn` |
| `finetype-cli` | CLI binary: `infer`, `generate`, `train`, `taxonomy` | `clap`, `finetype-core`, `finetype-model` |
| `finetype-duckdb` | DuckDB extension (planned) | `duckdb-extension-framework`, `finetype-core`, `finetype-model` |

### Key Types

| Type | Crate | Description |
|------|-------|-------------|
| `Taxonomy` | core | Loads and queries 151 definitions from YAML files |
| `Definition` | core | Single type definition with transform, validation, metadata |
| `Label` | core | Parsed `domain.category.type` with accessors |
| `Designation` | core | Scope enum: `Universal`, `LocaleSpecific`, `BroadNumbers`, etc. |
| `Generator` | core | Synthetic data generator for all 151 types |
| `Sample` | core | Generated `{text, label}` pair for training |
| `Tokenizer` | core | HuggingFace BERT tokenizer wrapper |

---

## Development Roadmap

### Phase 1: Taxonomy -- DONE

- [x] Design YAML spec with transformation contracts
- [x] Draft all 6 domains: `datetime` (46), `technology` (34), `identity` (25), `geography` (16), `representation` (19), `container` (11)
- [x] Resolve all duplicates across domains (canonical name + aliases)
- [x] Assign `broad_type`, `transform`, `validation` to every definition
- [x] Implement `Taxonomy` parser with `from_file` and `from_directory` loaders

### Phase 2: Data Generation -- DONE

- [x] Implement `Generator` with synthetic data for all 151 types
- [x] Verify 1:1 alignment between generator match arms and YAML definition keys
- [x] Add workspace dependencies: `fake`, `base64`, `sha2`, `md-5`, `glob`
- [ ] Add [`phonenumber`](https://crates.io/crates/phonenumber) for per-country phone generation
- [ ] Add [`luhn`](https://github.com/pacak/luhn) for credit card / IMEI checksum validation
- [ ] Generate training data with full `domain.category.type.locale` labels

### Phase 3: Build & Train -- NEXT

- [ ] Install Rust toolchain and verify `cargo build` succeeds
- [ ] Run `cargo test --all` to validate taxonomy parser and generator
- [ ] Generate training dataset: `finetype generate --samples 500 --output data/train.ndjson`
- [ ] Train CharCNN model on 151-label taxonomy
- [ ] Benchmark: single-value accuracy, inference latency
- [ ] Train tiered models (Tier 0 broad type, Tier 1 per-category)
- [ ] Implement column-mode inference with distribution-based disambiguation

### Phase 4: Validation Engine

- [ ] Integrate [`jsonschema`](https://github.com/Stranger6667/jsonschema) crate for JSON Schema validation
- [ ] Implement `finetype validate` CLI command with quality statistics
- [ ] Quarantine/null options for invalid rows
- [ ] Export validation reports

### Phase 5: DuckDB Extension

- [ ] Set up `finetype-duckdb` crate using [Rust extension template](https://github.com/duckdb/extension-template-rs)
- [ ] Implement `finetype(col)` -- single-value type detection
- [ ] Implement `finetype_profile(col)` -- column profiling with stats
- [ ] Implement `finetype_unpack(col)` -- recursive decomposition SQL generation
- [ ] Implement `finetype_cast(col)` -- automatic type casting
- [ ] Embed taxonomy + model weights at compile time
- [ ] Test extension-aware transforms (inet, spatial, monetary, netquack)

### Phase 6: Open Source & HuggingFace

- [ ] Publish under `noon-org/finetype` on GitHub (public)
- [ ] Upload model artifacts to HuggingFace (`noon-org/finetype-char-cnn`)
- [ ] Upload training dataset to HuggingFace Datasets
- [ ] Write model card with benchmarks and limitations
- [ ] Publish `finetype-cli` to crates.io
- [ ] Submit DuckDB extension to community extensions

---

## Architecture Decisions

### Why Character-Level CNN?

Format types are defined by character patterns (colons in MACs/IPv6, `@` in emails, dashes in UUIDs, `T` separator in ISO 8601). Character-level models capture these patterns directly without tokenization overhead.

### Why Tiered Inference?

A flat model with 500+ classes has poor accuracy and is expensive to retrain. Tiered inference gives:
- Better accuracy (each model distinguishes fewer classes)
- Faster inference (most strings resolved in 2 hops)
- Independent retraining (adding a date format only retrains the DATE tier)
- Extensibility (new domains are new branches, not a retrain of everything)

### Why Full Taxonomy Paths?

1. **Tiered inference** -- domain and category enable the model graph
2. **Filtering** -- `SELECT * WHERE finetype LIKE 'datetime.%'`
3. **Locale awareness** -- `phone_number.EN_AU` vs `phone_number.DE`
4. **Transformation lookup** -- the full path uniquely identifies the transform function

### Why Candle (not Burn)?

- Pure Rust, no Python runtime, no external C++ dependencies
- Integrates cleanly with DuckDB extension (single binary)
- Good Metal/CUDA support
- `hughcameron/finetype` (v1) used Burn+LibTorch -- useful for experimentation but LibTorch dependency is heavy for distribution

### Why DuckDB Extension + CLI on Same Codebase?

Both are thin wrappers around `finetype-core` and `finetype-model`. The CLI loads taxonomy/models from disk; the extension embeds them at compile time via `include_bytes!`. No conflict -- different build targets, different artifacts, shared logic.

---

## Development Commands

```bash
# Build CLI
cargo build --release

# Run inference
./target/release/finetype infer -i "192.168.1.1" --model models/default

# Generate training data
./target/release/finetype generate --samples 500 --output data/train.ndjson

# Train CharCNN model
./target/release/finetype train --data data/train.ndjson --model-type char-cnn --epochs 10

# Show taxonomy
./target/release/finetype taxonomy

# Show taxonomy for a specific domain
./target/release/finetype taxonomy --domain datetime

# Run tests
cargo test --all
```

---

## Related Repositories

- **noon-org/finetype** (this repo) -- Production codebase. Candle-based, DuckDB integration.
- **hughcameron/finetype** -- v1 experiments. Burn+LibTorch training, Python data generation with mimesis, HuggingFace dataset upload pipeline (`hughcameron/finetype_01`).

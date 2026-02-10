# FineType

Semantic type classification for text data. FineType profiles strings beyond primitive types, classifying them into a rich taxonomy of semantic categories.

```
$ finetype infer "192.168.1.1"
technology.internet.ip_v4

$ finetype infer "2024-01-15T10:30:00Z"
datetime.timestamp.iso_8601

$ finetype infer "hello@example.com"
identity.person.email
```

## Features

- **151 semantic types** across 6 domains — dates, times, IPs, emails, UUIDs, credit cards, and more
- **Locale-aware** — handles region-specific formats (16+ locales for dates, addresses, phone numbers)
- **Fast inference** — Character-level CNN model (600+ classifications/sec, 8.5 MB memory)
- **DuckDB integration** — use directly in SQL with `finetype()` and `finetype_profile()` scalar functions
- **Pure Rust** — no Python runtime required
- **Production-ready** — 33 tests, comprehensive taxonomy validation, benchmarked performance

## Installation

### Homebrew (macOS)

```bash
brew install noon-org/tap/finetype
```

### Cargo

```bash
cargo install finetype-cli
```

### From Source

```bash
git clone https://github.com/noon-org/finetype
cd finetype
cargo build --release
./target/release/finetype --version
```

## Usage

### CLI

```bash
# Single input
finetype infer -i "bc89:60a9:23b8:c1e9:3924:56de:3eb1:3b90"

# From file (one value per line)
finetype infer -f data.txt

# JSON output
finetype infer -i "test@example.com" --output json

# Generate training data
finetype generate --samples 1000 --output training.ndjson
```

### DuckDB Extension

```sql
-- Install extension
INSTALL finetype FROM community;
LOAD finetype;

-- Classify a single value
SELECT finetype('192.168.1.1');
-- → 'technology.internet.ip_v4'

-- Classify a column
SELECT value, finetype(value) as type
FROM my_table;
```

### As a Library

```rust
use finetype_model::Classifier;

let classifier = Classifier::load("models/default")?;
let result = classifier.classify("hello@example.com")?;

println!("{} (confidence: {:.2})", result.label, result.confidence);
// → identity.person.email (confidence: 0.97)
```

## Taxonomy

FineType recognizes **151 types** across **6 domains**:

| Domain | Types | Examples |
|--------|-------|----------|
| `datetime` | 46 | ISO 8601, RFC 2822, Unix timestamps, timezones, date formats |
| `technology` | 34 | IPv4, IPv6, MAC addresses, URLs, UUIDs, hashes, user agents |
| `identity` | 25 | Names, emails, phone numbers, passwords, gender symbols |
| `representation` | 19 | Integers, floats, booleans, hex colors, base64, JSON |
| `geography` | 16 | Latitude, longitude, countries, cities, postal codes |
| `container` | 11 | JSON objects, CSV rows, query strings, key-value pairs |

Each type is a **transformation contract** — if the model predicts `datetime.date.us_slash`, that guarantees `strptime(value, '%m/%d/%Y')::DATE` will succeed.

See [`labels/`](labels/) for the complete taxonomy (YAML definitions with validation schemas, transforms, and sample data).

## Performance

- **Model load time**: 66 ms (cold), 25–30 ms (warm)
- **Single inference**: p50=26 ms, p95=41 ms (includes CLI startup)
- **Batch throughput**: 600–750 values/sec on CPU
- **Memory footprint**: 8.5 MB peak RSS

## Architecture

**Three crates:**

```
crates/finetype-core/    # Taxonomy, tokenizer, synthetic data generation, 33 tests
crates/finetype-model/   # Candle CharCNN, trainer, inference engine
crates/finetype-cli/     # Binary: CLI commands (infer, generate, train, check)
```

**Data & Models:**

```
labels/                  # Taxonomy definitions (151 types, 6 domains, YAML)
models/char-cnn-v1/      # Pre-trained weights, config, label mapping
backlog/                 # Project tasks and decisions (Backlog.md format)
.github/workflows/       # CI/CD: fmt, clippy, test, finetype check; release cross-compile
```

**DuckDB Extension (Phase 5):**

```
finetype-duckdb/         # Rust extension template, embed models/taxonomy
```

## Model Training

```bash
# Generate training data
finetype generate --samples 75500 --priority 3 --output data/train.ndjson

# Train model
finetype train --data data/train.ndjson --epochs 10 --batch-size 64 --device cpu
```

## Development

See [`DEVELOPMENT.md`](DEVELOPMENT.md) for:

- Complete taxonomy documentation
- Architecture decisions (tiered models, column-mode inference, DuckDB extension strategy)
- Roadmap (Phases 1–6: data gen → training → validation → DuckDB ext → HuggingFace release)
- Testing and benchmarking

### Quick Start

```bash
# Install Rust, clone repo
git clone https://github.com/noon-org/finetype
cd finetype

# Run tests
cargo test --all

# Validate taxonomy
cargo run --release -- check

# Infer a type
cargo run --release -- infer "hello@example.com"
```

## License

MIT — see [`LICENSE`](LICENSE)

## Contributing

Contributions welcome! Please open an issue or PR.

## Credits

Part of the [Noon](https://github.com/noon-org) project.

Built with:
- [Candle](https://github.com/huggingface/candle) — Rust ML framework
- [DuckDB](https://duckdb.org) — Analytical database
- [Serde](https://serde.rs) — Serialization
- [Regex](https://docs.rs/regex) — Pattern matching

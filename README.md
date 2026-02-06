# FineType

Semantic type classification for text data. FineType profiles strings beyond primitive types, classifying them into a rich taxonomy of semantic categories.

```
$ finetype infer "192.168.1.1"
internet.ip_v4

$ finetype infer "2024-01-15T10:30:00Z"
datetime.iso_8601

$ finetype infer "hello@example.com"
person.email
```

## Features

- **100+ semantic types** — dates, IPs, emails, UUIDs, credit cards, and more
- **Locale-aware** — handles region-specific formats
- **Fast inference** — Rust + Candle transformer model
- **DuckDB integration** — use directly in SQL queries
- **Pure Rust** — no Python runtime required

## Installation

```bash
cargo install finetype-cli
```

Or build from source:

```bash
git clone https://github.com/noon-org/finetype
cd finetype
cargo build --release
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
-- → 'internet.ip_v4'

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
// → person.email (confidence: 0.97)
```

## Taxonomy

FineType recognizes types across these categories:

| Category | Examples |
|----------|----------|
| `datetime` | ISO 8601, RFC 2822, Unix timestamps, timezones |
| `internet` | IPv4, IPv6, MAC addresses, URLs, emails, user agents |
| `cryptographic` | UUIDs, hashes (MD5, SHA), Bitcoin/Ethereum addresses |
| `person` | Names, phone numbers, emails |
| `address` | Cities, countries, postal codes, coordinates |
| `code` | EAN, IMEI, ISBN, ISSN, locale codes |
| `payment` | Credit card numbers, crypto addresses |
| `finance` | Currency codes, stock tickers |
| `science` | DNA/RNA sequences |

See [`labels/definitions.yaml`](labels/definitions.yaml) for the complete taxonomy.

## Architecture

```
finetype/
├── crates/
│   ├── finetype-core/    # Taxonomy, tokenizer, data generation
│   ├── finetype-model/   # Candle transformer model
│   └── finetype-cli/     # CLI binary
├── extension/            # DuckDB extension
├── labels/               # Taxonomy definitions (YAML)
└── models/               # Pre-trained weights
```

## Training

```bash
# Generate training data
finetype generate --samples 10000 --priority 3 --output data/train.ndjson

# Train model
finetype train --data data/train.ndjson --epochs 5 --device metal
```

## License

MIT

## Credits

Part of the [Noon](https://github.com/noon-org) project.

Built with:
- [Candle](https://github.com/huggingface/candle) — ML framework
- [DuckDB](https://duckdb.org) — Analytical database

# FineType v2 Taxonomy — Quick Reference

## Overview

**172 Type Definitions** organized in **6 Domains** across **29 Categories**.

Each type is a transformation contract:
```yaml
domain.category.type:
  broad_type: VARCHAR|BIGINT|DOUBLE|BOOLEAN|DATE|TIMESTAMP|UUID|JSON|INET
  format_string: strptime format or null
  transform: DuckDB SQL expression ({col} = column placeholder)
  validation: JSON Schema fragment for data validation
```

## Domain Summary

| Domain | Types | Categories | Purpose |
|--------|-------|------------|---------|
| **datetime** | 46 | 7 | Temporal formats: timestamps, dates, times, epochs, durations |
| **technology** | 41 | 5 | Technical formats: networks, crypto, code, dev, hardware |
| **identity** | 32 | 3 | Personal data: person, payment, academic |
| **geography** | 16 | 5 | Location data: places, addresses, coordinates, transport |
| **representation** | 24 | 4 | Data formats: numeric, text, files, scientific |
| **container** | 13 | 3 | Nested data: objects, arrays, key-value pairs |

## Key Features

### ✅ Transformation Contracts

Each type guarantees successful casting. Example:

```yaml
datetime.date.us_slash:
  broad_type: DATE
  format_string: "%m/%d/%Y"
  transform: "strptime({col}, '%m/%d/%Y')::DATE"
  validation:
    type: string
    pattern: "^(0[1-9]|1[0-2])/([0-2][0-9]|3[0-1])/\\d{4}$"
```

### ✅ Locale Support

Universal types (same format everywhere):
```yaml
technology.internet.ipv4:
  designation: universal
  locales: [UNIVERSAL]
```

Locale-specific types (expand per-locale):
```yaml
identity.person.phone_number:
  designation: locale_specific
  locales: [EN, EN_AU, EN_US, DE, FR, ES, ...]
```

### ✅ Decomposition (Struct Expansion)

Extract nested fields:
```yaml
identity.person.full_name:
  decompose:
    first_name: "REGEXP_EXTRACT({col}, '^([^ ]+)')"
    last_name: "REGEXP_EXTRACT({col}, '([^ ]+)$')"
```

### ✅ Recursive Inference (Containers)

JSON objects trigger field-level classification:
```yaml
container.object.json:
  designation: universal
  decompose: "RECURSIVE_INFER_ON_FIELDS({col})"
  # Input: {"user": "john", "age": 30}
  # Output: Struct<user: VARCHAR, age: BIGINT>
```

### ✅ Release Priority Tiers

**Priority 5** (Immediate): ISO dates, UUIDs, IPv4 — high value, low variance
**Priority 4** (First Pass): US dates, emails, phone — common, well-defined
**Priority 3** (Second Pass): ISBN, URLs, timestamps — less common
**Priority 2** (Extended): XML, YAML, scientific — specialized
**Priority 1** (Later): Broad types, passwords, occupations — high variance
**Priority 0** (Deprioritized): Generic text, MIME types — very difficult

### ✅ broad_* Types

Fully specified with lower priority (train later as confidence improves):
- `broad_numbers`: numeric.increment (auto-increment IDs)
- `broad_words`: internet.http_method, development.programming_language
- `broad_characters`: file.mime_type, text.plain_text
- `broad_object`: (reserved for complex nested)

## Domain Details

### 1. DATETIME (46 types, 7 categories)

**timestamp** (12 types)
- iso_8601, rfc_3339, rfc_2822, american, european, etc.

**date** (17 types)
- us_slash, eu_dot, iso_8601, short_ymd, numeric_mdy, etc.

**time** (5 types)
- iso_8601, 12_hour, 24_hour, unix_epoch_ms, etc.

**epoch** (3 types)
- unix_timestamp, unix_ms, unix_ns

**offset** (2 types)
- gmt_offset, timezone

**duration** (1 type)
- iso_8601_interval

**component** (6 types)
- year, month, day, day_of_week, hour, minute

### 2. TECHNOLOGY (41 types, 5 categories)

**internet** (13 types)
- ipv4, ipv6, mac_address, url, uri, hostname, port, tld, slug, user_agent, http_method, http_status_code, ip_v4_with_port

**cryptographic** (4 types)
- uuid, hash, token_hex, token_urlsafe

**code** (5 types)
- isbn, imei, ean, issn, locale_code, pin

**development** (8 types)
- version, calver, programming_language, software_license, stage, os, boolean

**hardware** (6 types)
- cpu, ram_size, screen_size, generation

### 3. IDENTITY (32 types, 3 categories)

**person** (20 types)
- full_name, first_name, last_name, email, phone_number, username, password, gender, gender_code, gender_symbol, nationality, blood_type, height, weight, age, occupation

**payment** (8 types)
- credit_card_number, credit_card_expiration_date, cvv, credit_card_network, bitcoin_address, ethereum_address, paypal_email

**academic** (4 types)
- degree, university

### 4. GEOGRAPHY (16 types, 5 categories)

**location** (4 types)
- country, country_code, continent, region

**address** (5 types)
- full_address, street_number, street_name, street_suffix, postal_code

**coordinate** (3 types)
- latitude, longitude, coordinates

**transportation** (2 types)
- iata_code, icao_code

**contact** (1 type)
- calling_code

### 5. REPRESENTATION (24 types, 4 categories)

**numeric** (6 types)
- integer_number, decimal_number, scientific_notation, percentage, increment

**text** (7 types)
- plain_text, sentence, word, color_hex, color_rgb, emoji

**file** (3 types)
- extension, mime_type, file_size

**scientific** (8 types)
- dna_sequence, rna_sequence, protein_sequence, measurement_unit, metric_prefix

### 6. CONTAINER (13 types, 3 categories)

**object** (5 types)
- json, json_array, xml, yaml, csv

**array** (4 types)
- comma_separated, pipe_separated, semicolon_separated, whitespace_separated

**key_value** (2 types)
- query_string, form_data

## Label Format

```
{domain}.{category}.{type}.{locale}
```

Examples:
- `datetime.timestamp.iso_8601.UNIVERSAL`
- `datetime.date.us_slash.UNIVERSAL`
- `identity.person.phone_number.EN_AU`
- `technology.internet.ipv4.UNIVERSAL`
- `geography.location.country.FR`

## YAML Specification

See `DEVELOPMENT.md` for complete YAML schema documentation.

Quick reference:

```yaml
domain.category.type:
  # Identity
  title: "Human-readable name"
  description: >
    What this format is, where it's used, any special considerations
  designation: universal | locale_specific | broad_numbers | broad_words | broad_characters | broad_object
  locales: [UNIVERSAL] or [EN, FR, DE, ...]

  # Transformation Contract
  broad_type: VARCHAR | BIGINT | DOUBLE | BOOLEAN | DATE | TIMESTAMP | UUID | JSON | INET | POINT | ...
  format_string: "%Y-%m-%d" or null
  transform: "SQL expression with {col} placeholder"
  transform_ext: "SQL with extension or null"
  decompose: "SQL to extract nested fields or null"

  # Validation
  validation:
    type: string | number | boolean | ...
    pattern: "regex pattern"
    enum: [value1, value2, ...]
    minLength: 1
    maxLength: 100
    minimum: 0
    maximum: 150

  # Inference
  tier: [BROAD_TYPE, category]
  release_priority: 0-5

  # Metadata
  aliases: [alias1, alias2] or null
  samples: ["example1", "example2", ...]
  references: null
  notes: "v1 migration notes, design decisions, etc."
```

## File Locations

```
labels/
├── definitions_v2_datetime.yaml
├── definitions_v2_technology.yaml
├── definitions_v2_identity.yaml
├── definitions_v2_geography.yaml
├── definitions_v2_representation.yaml
└── definitions_v2_container.yaml

DEVELOPMENT.md          # Complete specification & roadmap
TAXONOMY_QUICK_REFERENCE.md  # This file
```

## Common Transformations

### String → Date
```sql
strptime('01/15/2024', '%m/%d/%Y')::DATE
```

### String → Timestamp
```sql
strptime('2024-01-15T10:30:00Z', '%Y-%m-%dT%H:%M:%SZ')
```

### String → UUID
```sql
CAST('550e8400-e29b-41d4-a716-446655440000' AS UUID)
```

### String → Boolean
```sql
CAST(value AS BOOLEAN)  -- Accepts: true, false, 1, 0, yes, no, on, off
```

### String → INET (requires inet extension)
```sql
CAST('192.168.1.1' AS INET)
```

### Container → Struct (recursive inference)
```sql
-- Input: {"user": "john", "age": 30}
-- Output: Struct<user: VARCHAR, age: BIGINT>
```

## Migration from v1

Each v2 type includes:
- **aliases**: v1 names for cross-reference
- **notes**: Migration notes and rationale

Example:
```yaml
technology.internet.ipv4:
  aliases: [ipv4]
  notes: >
    v1 migration: Was internet.ip_v4. transform_ext uses DuckDB's inet extension
    if available. Without the extension, stored as VARCHAR but validated as IPv4.
```

## Next Steps

See `DEVELOPMENT.md` for Phase 2 and beyond:
1. Data generation for all 172 types
2. Model training (Tier 0 → Tier 1 → Tier 2)
3. DuckDB extension integration
4. HuggingFace model & dataset release

---

**Last Updated**: 2026-02-09
**Commit**: 1f60bd7
**Status**: Phase 1 Complete ✅

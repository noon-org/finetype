---
id: NNFT-005
title: Fix generator quality issues identified by char-cnn-v1 confusion analysis
status: To Do
assignee: []
created_date: '2026-02-10 05:30'
updated_date: '2026-02-10 06:50'
labels:
  - data-generation
  - model-quality
milestone: 'Phase 2: Data Generation'
dependencies:
  - NNFT-004
  - NNFT-007
references:
  - crates/finetype-core/src/generator.rs
  - models/char-cnn-v1/eval_results.json
priority: high
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
The char-cnn-v1 evaluation revealed several classes with very poor recall driven by generator or taxonomy issues. This task addresses the generator-side fixes:

**Critical confusions (generator fixes):**
- `gender_symbol` → `emoji` (0% recall): Generator likely emits emoji codepoints rather than gender symbols (♂/♀/⚧). Fix to produce only Unicode gender symbols.
- `credit_card_number` → `imei` (9% recall): Both produce long digit strings. Add `luhn` crate for valid Luhn-checksum credit card numbers with correct prefix patterns (Visa 4xxx, MC 5[1-5]xx, etc.).
- `email` → `paypal_email` (51% recall): paypal_email generator likely produces generic emails. Ensure paypal_email samples include PayPal-specific patterns while email uses diverse domains.
- `token_urlsafe` → `bitcoin_address` (50% recall): Ensure token_urlsafe uses standard base64url chars while bitcoin uses base58 with correct prefix (1/3/bc1).
- `pin` (4% recall): Generator should produce 4-6 digit PINs only, not overlap with street numbers.
- `port` (3% recall): Constrain to common port numbers (80, 443, 22, 3306, 8080, etc.) to distinguish from generic incrementing integers.

**Numeric disambiguation:**
- `postal_code` → `increment` (44% recall): Ensure postal codes use realistic country-specific formats (US 5-digit, UK alphanumeric, etc.).
- `epoch.unix_microseconds` → `imei` (29% recall): Ensure epoch values are in valid microsecond ranges (13-16 digits with appropriate magnitude).
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 gender_symbol generator produces only Unicode gender symbols (♂ ♀ ⚧), not emoji
- [ ] #2 credit_card_number generator uses luhn crate for valid checksums with network-specific prefixes
- [ ] #3 paypal_email samples include PayPal-distinctive patterns (paypal.com, pp- prefix, etc.)
- [ ] #4 token_urlsafe uses base64url alphabet only; bitcoin_address uses base58 with 1/3/bc1 prefix
- [ ] #5 pin generator constrained to 4-6 digits; port generator uses common port number distribution
- [ ] #6 postal_code generator produces country-specific formats
- [ ] #7 epoch generators produce values in correct magnitude ranges for seconds/ms/μs
- [ ] #8 Regenerated training + test datasets pass checker validation (151/151)
<!-- AC:END -->

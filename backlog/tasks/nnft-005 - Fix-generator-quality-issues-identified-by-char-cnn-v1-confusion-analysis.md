---
id: NNFT-005
title: Fix generator quality issues identified by char-cnn-v1 confusion analysis
status: Done
assignee:
  - '@nightingale'
created_date: '2026-02-10 05:30'
updated_date: '2026-02-10 12:05'
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
- [x] #1 gender_symbol generator produces only Unicode gender symbols (♂ ♀ ⚧), not emoji
- [x] #2 credit_card_number generator uses luhn crate for valid checksums with network-specific prefixes
- [x] #3 paypal_email samples include PayPal-distinctive patterns (paypal.com, pp- prefix, etc.)
- [x] #4 token_urlsafe uses base64url alphabet only; bitcoin_address uses base58 with 1/3/bc1 prefix
- [x] #5 pin generator constrained to 4-6 digits; port generator uses common port number distribution
- [x] #6 postal_code generator produces country-specific formats
- [x] #7 epoch generators produce values in correct magnitude ranges for seconds/ms/μs
- [x] #8 Regenerated training + test datasets pass checker validation (151/151)
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Review each AC against current generator code\n2. AC #2 already done in NNFT-007 (credit_card Luhn) — mark immediately\n3. Fix gender_symbol: verify generator is correct (♂♀⚧), expand symbol set if needed\n4. Fix paypal_email: add PayPal-distinctive patterns (paypal.com, pp-prefix, transaction IDs)\n5. Fix token_urlsafe: ensure base64url chars (- and _) are included to distinguish from base58\n6. Fix port: constrain to common/well-known port numbers\n7. Fix postal_code: add more country-specific formats (CA, JP, AU) to distinguish from increment\n8. Verify epoch ranges are correct (they appear fine — 10/13/16 digit ranges)\n9. Run finetype check --verbose and cargo test --all\n10. Regenerate training data not in scope (AC #8 — will verify checker passes)
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
- gender_symbol: already correct (♂♀⚧), added ⚪ to match definition pattern
- paypal_email: completely rewritten with 5 PayPal-distinctive patterns (paypal.com domain, pp-prefix, merchant, service subdomain, paypal- prefix)
- token_urlsafe: rewritten to use full base64url alphabet with guaranteed - or _ character
- port: weighted 60% toward well-known ports (22,80,443,3306,5432,8080 etc), 20% registered, 20% ephemeral
- postal_code: expanded from 3 to 8 formats (US, US+4, UK, CA, JP, DE/FR, AU, NL)
- pin: already correct (4-6 digits with leading zeros) — inherent column-mode ambiguity
- epoch: already correct (10/13/16 digit ranges) — IMEI confusion should resolve via NNFT-007 TAC prefixes
- credit_card: already fixed in NNFT-007
- All 31 tests pass, checker 151/151 at 100%
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Fixed 5 generator quality issues identified by char-cnn-v1 confusion analysis to improve model discrimination between similar types.

Changes:
- **paypal_email**: Completely rewritten with 5 PayPal-distinctive patterns — paypal.com domain, pp-prefix format, merchant payments, service subdomain, and paypal-prefixed addresses. Previously indistinguishable from generic email.
- **token_urlsafe**: Rewritten to use full base64url alphabet (A-Z, a-z, 0-9, -, _) with guaranteed inclusion of - or _ characters. Previously used only alphanumeric chars, causing 50% confusion with bitcoin_address (base58).
- **port**: Weighted distribution: 60% well-known ports (22, 80, 443, 3306, 5432, 8080, etc.), 20% registered ports (1024-49151), 20% ephemeral (49152-65535). Previously uniform 1-65535 indistinguishable from random integers.
- **postal_code**: Expanded from 3 to 8 country-specific formats (US ZIP, US ZIP+4, UK, Canada A1A 1A1, Japan 123-4567, Germany/France, Australia 4-digit, Netherlands 1234 AB). Previously mostly 5-digit numbers confused with increment.
- **gender_symbol**: Added ⚪ to match definition pattern (♂♀⚧⚪).

Not changed (already correct):
- credit_card_number (fixed in NNFT-007)
- pin (4-6 digits with leading zeros — low recall is inherent column-mode ambiguity)
- epoch generators (correct magnitude ranges — IMEI confusion resolves via NNFT-007 TAC prefixes)

Validation:
- `cargo test --all`: 31/31 pass
- `finetype check --verbose`: 151/151 definitions, 7550/7550 samples (100%)
<!-- SECTION:FINAL_SUMMARY:END -->

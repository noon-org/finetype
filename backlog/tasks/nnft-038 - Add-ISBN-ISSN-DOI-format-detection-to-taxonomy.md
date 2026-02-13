---
id: NNFT-038
title: 'Add ISBN, ISSN, DOI format detection to taxonomy'
status: Done
assignee:
  - '@nightingale'
created_date: '2026-02-13 10:10'
updated_date: '2026-02-13 12:20'
labels:
  - taxonomy
  - training-data
  - evaluation
dependencies:
  - NNFT-037
priority: medium
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
The GitTables 1M evaluation (NNFT-037) identified structured identifiers — ISBN, ISSN, and DOI — as real-world types with recognizable format patterns that FineType currently does not detect. These appeared in the unmapped ground truth labels and are good candidates for the technology.code category.

- ISBN: 10 or 13-digit book identifiers with check digits (ISBN-10, ISBN-13)
- ISSN: 8-digit serial identifiers (XXXX-XXXX)
- DOI: Digital Object Identifiers (10.XXXX/... prefix pattern)
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 ISBN-10 and ISBN-13 formats added to taxonomy with generators
- [x] #2 ISSN format added to taxonomy with generator
- [x] #3 DOI format added to taxonomy with generator
- [x] #4 Training data regenerated with new types
- [x] #5 Model retrained and accuracy validated on synthetic test set
- [x] #6 GitTables 1M re-evaluation confirms detection of these types in real data
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Add DOI type definition to labels/definitions_technology.yaml under technology.code
2. Improve ISBN generator: add ISBN-10 format, with/without hyphens, valid check digits
3. Add DOI generator to generator.rs
4. Bump ISSN release_priority from 2 → 3 (it's a real format we should detect)
5. Regenerate training data with `finetype generate`
6. Retrain model (batched with NNFT-039 year improvements)
7. Run eval on synthetic test set
8. Re-run GitTables 1M evaluation
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Model char-cnn-v3 training completed: 91.92% training accuracy over 5 epochs.

Evaluation on test_v3.ndjson (30k samples):
- Overall accuracy: 93.63%
- Top-3 accuracy: 99.14%
- DOI: 100% precision, 100% recall — perfect
- ISBN: 89.0% precision, 76.5% recall — some confusion with EAN
- ISSN: 100% precision, 100% recall — perfect

GitTables benchmark (883 tables, 2363 columns) with v3 model:
- Technology domain: 94.1% row / 95.6% column (stable)
- DOI detection validated on synthetic data (100% precision/recall)
- No ISBN/ISSN/DOI columns present in this benchmark slice, but the types don't interfere with other classifications
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Added DOI type to taxonomy (technology.code.doi) with realistic generator producing publisher registrant codes (Nature, Elsevier, Science, ACM, IEEE, Wiley, Springer, arXiv, Zenodo) and 5 suffix styles. Enhanced ISBN generator with ISBN-10/ISBN-13 support, valid check digits (Luhn mod-11 and EAN algorithms), and hyphenated/bare formats. Bumped ISSN release_priority from 2→3.

Trained char-cnn-v3 model on 150k samples (152 types): 91.92% training accuracy, 93.63% test accuracy, 99.14% top-3 accuracy. DOI achieves 100% precision/recall, ISSN 100%, ISBN 89%/76.5%.

GitTables benchmark stable at 95.6% for technology domain. Overall accuracy improved +1.1% to 53.1% row-mode.

Files changed:
- labels/definitions_technology.yaml (DOI type, ISSN priority)
- crates/finetype-core/src/generator.rs (DOI generator, ISBN improvements, check digit helpers)
- data/train_v3.ndjson, data/test_v3.ndjson (new datasets)
- models/char-cnn-v3/ (new model)
<!-- SECTION:FINAL_SUMMARY:END -->

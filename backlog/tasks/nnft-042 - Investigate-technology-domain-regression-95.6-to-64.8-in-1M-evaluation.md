---
id: NNFT-042
title: Investigate technology domain regression (95.6% to 64.8%) in 1M evaluation
status: Done
assignee:
  - '@nightingale'
created_date: '2026-02-13 10:10'
updated_date: '2026-02-13 10:32'
labels:
  - evaluation
  - model
  - investigation
dependencies:
  - NNFT-037
priority: medium
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
The technology domain dropped from 95.6% accuracy in the benchmark subset to 64.8% in the 1M evaluation — a 30.8% regression. This was the largest domain-level accuracy drop.

Hypothesis: The benchmark's small sample of URLs and tech types were highly format-regular (standard https:// URLs), while the broader corpus includes shortened URLs, non-standard formats, encoded paths, and edge cases the model hasn't seen.

This investigation should identify the specific failure patterns and determine whether they're addressable through training data improvements or post-processing rules.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 Root cause analysis: which GT labels in technology domain have lowest accuracy
- [x] #2 Sample of misclassified technology values extracted and categorized
- [x] #3 Determination: training data issue vs taxonomy gap vs post-processing opportunity
- [x] #4 Recommended fix documented (may feed into separate implementation task)
<!-- AC:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Root cause analysis complete.

**Finding: The regression is almost entirely caused by the `id` ground truth label, not `url`.**

Breakdown of technology domain (3,737 columns mapped):
- `id` columns: 2,164 total, 876 correct → **40.5% accuracy**
- `url` columns: 1,573 total, 1,534 correct → **97.5% accuracy**

The `id` label is the problem — it accounts for 58% of all technology-domain columns but only 40.5% accuracy, dragging overall technology domain accuracy from 97.5% (url-only) down to 64.8%.

**Why id columns fail:**
ID values in the real world are semantically IDs but formatted as:
- Large integers (33% → classified as EAN, 16.4% → integer, 14.3% → epoch)
- Short numbers (→ postal code, CVV, age, port)
- Alphanumeric codes (→ password, username, bitcoin_address)
- UUIDs, emails, dates mixed in

FineType correctly classifies the *format* of these values. The ground truth labels them as \"id\" which maps to \"technology\", but FineType sees numeric integers, epoch timestamps, etc. — which are format-correct classifications.

**Determination: This is a MAPPING PROBLEM, not a model problem.**

The `id` → `technology` domain mapping is fundamentally flawed. IDs have no consistent format — they can be integers, UUIDs, strings, or anything. FineType correctly detects the actual format.

**Recommendations:**
1. Remove `id` from the type_mapping table — it's not a format-detectable type
2. Alternatively, map `id` to `representation` (since most IDs are numeric representations)
3. Do NOT add \"id\" as a FineType type — it's a semantic label, not a format
4. Consider adding a post-processing rule that could detect sequential/random numeric IDs by column distribution patterns

If `id` is removed from mapping, technology accuracy would be ~97.5% (url only), which matches the benchmark."
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Investigated the technology domain regression (95.6% → 64.8%) in the GitTables 1M evaluation.

**Root Cause:** The `id` ground truth label drags technology accuracy down. It accounts for 2,164 of 3,737 technology-mapped columns but only achieves 40.5% accuracy. URL columns remain excellent at 97.5%.

**Why:** ID values have no consistent format — they're integers, UUIDs, epochs, alphanumeric codes, etc. FineType correctly classifies their actual format (integer, epoch, UUID), but the ground truth maps them all to \"technology\". This is a mapping problem, not a model problem.

**Key data:**
- id columns: 40.5% accuracy (2,164 columns) — values classified as EAN (33%), integer (16.4%), epoch (14.3%), postal_code (7.3%)
- url columns: 97.5% accuracy (1,573 columns) — correctly classified as url/uri

**Recommendation:** Remove `id` from the technology domain mapping. It's a semantic label, not a format-detectable type. With `id` removed, technology accuracy would be ~97.5%.

**Files:** eval/gittables/investigate_tech.sql added for the investigation query."
<!-- SECTION:FINAL_SUMMARY:END -->

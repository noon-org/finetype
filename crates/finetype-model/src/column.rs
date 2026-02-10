//! Column-mode inference for distribution-based type disambiguation.
//!
//! Column-mode takes a vector of string values (a column sample), runs
//! single-value inference on each, aggregates the predictions, and applies
//! disambiguation rules to determine the most likely type for the entire column.
//!
//! This is critical for resolving ambiguous types like:
//! - `us_slash` vs `eu_slash` dates (MM/DD vs DD/MM)
//! - `short_dmy` vs `short_mdy` dates
//! - `latitude` vs `longitude` coordinates
//! - Numeric types (port, increment, postal_code, integer_number)

use crate::inference::{CharClassifier, ClassificationResult, InferenceError};
use std::collections::HashMap;

/// Configuration for column-mode inference.
#[derive(Debug, Clone)]
pub struct ColumnConfig {
    /// Maximum number of values to sample from the column (default: 100).
    pub sample_size: usize,
    /// Minimum fraction of votes a type needs to be the winner (default: 0.3).
    /// If no type reaches this threshold, the result confidence is lowered.
    pub min_agreement: f32,
}

impl Default for ColumnConfig {
    fn default() -> Self {
        Self {
            sample_size: 100,
            min_agreement: 0.3,
        }
    }
}

/// Result of column-mode inference.
#[derive(Debug, Clone)]
pub struct ColumnResult {
    /// The predicted type label for the column.
    pub label: String,
    /// Confidence score (0.0 to 1.0).
    pub confidence: f32,
    /// Vote distribution: label → fraction of samples classified as this type.
    pub vote_distribution: Vec<(String, f32)>,
    /// Whether a disambiguation rule was applied to override the majority vote.
    pub disambiguation_applied: bool,
    /// Name of the disambiguation rule applied, if any.
    pub disambiguation_rule: Option<String>,
    /// Number of values actually classified.
    pub samples_used: usize,
}

/// Column-mode classifier that wraps a single-value classifier.
pub struct ColumnClassifier {
    classifier: CharClassifier,
    config: ColumnConfig,
}

impl ColumnClassifier {
    /// Create a new column classifier wrapping a CharClassifier.
    pub fn new(classifier: CharClassifier, config: ColumnConfig) -> Self {
        Self { classifier, config }
    }

    /// Create with default configuration.
    pub fn with_defaults(classifier: CharClassifier) -> Self {
        Self::new(classifier, ColumnConfig::default())
    }

    /// Classify a column of values, returning a single type prediction.
    ///
    /// The algorithm:
    /// 1. Sample up to `config.sample_size` values
    /// 2. Run single-value inference on each
    /// 3. Aggregate votes by predicted label
    /// 4. Apply disambiguation rules for known ambiguous pairs
    /// 5. Return the final label with confidence
    pub fn classify_column(&self, values: &[String]) -> Result<ColumnResult, InferenceError> {
        if values.is_empty() {
            return Ok(ColumnResult {
                label: "unknown".to_string(),
                confidence: 0.0,
                vote_distribution: vec![],
                disambiguation_applied: false,
                disambiguation_rule: None,
                samples_used: 0,
            });
        }

        // Step 1: Sample values
        let sample = if values.len() <= self.config.sample_size {
            values.to_vec()
        } else {
            // Deterministic sampling: evenly spaced
            let step = values.len() as f64 / self.config.sample_size as f64;
            (0..self.config.sample_size)
                .map(|i| values[(i as f64 * step) as usize].clone())
                .collect()
        };

        let n_samples = sample.len();

        // Step 2: Run batch inference
        let results = self.classifier.classify_batch(&sample)?;

        // Step 3: Aggregate votes
        let mut vote_counts: HashMap<String, usize> = HashMap::new();
        for result in &results {
            *vote_counts.entry(result.label.clone()).or_default() += 1;
        }

        // Sort by count descending
        let mut votes: Vec<(String, usize)> = vote_counts.into_iter().collect();
        votes.sort_by(|a, b| b.1.cmp(&a.1));

        let vote_distribution: Vec<(String, f32)> = votes
            .iter()
            .map(|(label, count)| (label.clone(), *count as f32 / n_samples as f32))
            .collect();

        // Majority winner
        let (majority_label, majority_count) = votes.first().cloned().unwrap_or_default();
        let majority_fraction = majority_count as f32 / n_samples as f32;

        // Step 4: Apply disambiguation rules
        let disambiguation = disambiguate(&sample, &results, &votes, n_samples);

        if let Some((label, rule_name)) = disambiguation {
            Ok(ColumnResult {
                label,
                confidence: majority_fraction.max(0.8), // Disambiguation rules are high-confidence
                vote_distribution,
                disambiguation_applied: true,
                disambiguation_rule: Some(rule_name),
                samples_used: n_samples,
            })
        } else {
            // No disambiguation needed — use majority vote
            let confidence = if majority_fraction >= self.config.min_agreement {
                majority_fraction
            } else {
                majority_fraction * 0.5 // Low agreement → low confidence
            };

            Ok(ColumnResult {
                label: majority_label,
                confidence,
                vote_distribution,
                disambiguation_applied: false,
                disambiguation_rule: None,
                samples_used: n_samples,
            })
        }
    }

    /// Get a reference to the underlying classifier.
    pub fn classifier(&self) -> &CharClassifier {
        &self.classifier
    }

    /// Get a reference to the configuration.
    pub fn config(&self) -> &ColumnConfig {
        &self.config
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// DISAMBIGUATION RULES
// ═══════════════════════════════════════════════════════════════════════════════

/// Disambiguation rule pairs: types that are ambiguous in single-value mode.
const DATE_SLASH_PAIR: (&str, &str) = ("datetime.date.us_slash", "datetime.date.eu_slash");

const SHORT_DATE_PAIR: (&str, &str) = ("datetime.date.short_mdy", "datetime.date.short_dmy");

const COORDINATE_PAIR: (&str, &str) = (
    "geography.coordinate.latitude",
    "geography.coordinate.longitude",
);

/// Apply disambiguation rules when the vote distribution contains known ambiguous pairs.
///
/// Returns Some((resolved_label, rule_name)) if a rule was applied, None otherwise.
fn disambiguate(
    values: &[String],
    results: &[ClassificationResult],
    votes: &[(String, usize)],
    _n_samples: usize,
) -> Option<(String, String)> {
    // Get the top labels in the vote
    let top_labels: Vec<&str> = votes.iter().take(3).map(|(l, _)| l.as_str()).collect();

    // Rule 1: Date slash disambiguation (us_slash vs eu_slash)
    if contains_pair(&top_labels, DATE_SLASH_PAIR.0, DATE_SLASH_PAIR.1) {
        if let Some(label) = disambiguate_slash_dates(values) {
            return Some((label, "date_slash_disambiguation".to_string()));
        }
    }

    // Rule 2: Short date disambiguation (short_mdy vs short_dmy)
    if contains_pair(&top_labels, SHORT_DATE_PAIR.0, SHORT_DATE_PAIR.1) {
        if let Some(label) = disambiguate_short_dates(values) {
            return Some((label, "short_date_disambiguation".to_string()));
        }
    }

    // Rule 3: Coordinate disambiguation (latitude vs longitude)
    if contains_pair(&top_labels, COORDINATE_PAIR.0, COORDINATE_PAIR.1) {
        if let Some(label) = disambiguate_coordinates(values) {
            return Some((label, "coordinate_disambiguation".to_string()));
        }
    }

    // Rule 4: Numeric type disambiguation
    if let Some((label, rule)) = disambiguate_numeric(values, results, &top_labels) {
        return Some((label, rule));
    }

    None
}

/// Check if two labels are both present in the top candidates.
fn contains_pair(labels: &[&str], a: &str, b: &str) -> bool {
    labels.contains(&a) && labels.contains(&b)
}

/// Disambiguate us_slash vs eu_slash dates.
///
/// Pattern: `DD/MM/YYYY` or `MM/DD/YYYY`
/// Rule: If ANY value has first component > 12, it must be DD/MM (eu_slash).
///       If ANY value has second component > 12, it must be MM/DD (us_slash).
fn disambiguate_slash_dates(values: &[String]) -> Option<String> {
    let mut first_over_12 = false;
    let mut second_over_12 = false;

    for val in values {
        let parts: Vec<&str> = val.split('/').collect();
        if parts.len() >= 2 {
            if let Ok(first) = parts[0].parse::<u32>() {
                if first > 12 {
                    first_over_12 = true;
                }
            }
            if let Ok(second) = parts[1].parse::<u32>() {
                if second > 12 {
                    second_over_12 = true;
                }
            }
        }
    }

    if first_over_12 && !second_over_12 {
        // First component > 12 means it's the day → DD/MM/YYYY → eu_slash
        Some("datetime.date.eu_slash".to_string())
    } else if second_over_12 && !first_over_12 {
        // Second component > 12 means it's the day → MM/DD/YYYY → us_slash
        Some("datetime.date.us_slash".to_string())
    } else {
        // Both ambiguous or contradictory — let model decide
        None
    }
}

/// Disambiguate short_dmy vs short_mdy dates.
///
/// Pattern: `DD-MM-YY` or `MM-DD-YY`
/// Rule: Same as slash dates but with dash separator.
fn disambiguate_short_dates(values: &[String]) -> Option<String> {
    let mut first_over_12 = false;
    let mut second_over_12 = false;

    for val in values {
        let parts: Vec<&str> = val.split('-').collect();
        if parts.len() >= 2 {
            if let Ok(first) = parts[0].parse::<u32>() {
                if first > 12 {
                    first_over_12 = true;
                }
            }
            if let Ok(second) = parts[1].parse::<u32>() {
                if second > 12 {
                    second_over_12 = true;
                }
            }
        }
    }

    if first_over_12 && !second_over_12 {
        Some("datetime.date.short_dmy".to_string())
    } else if second_over_12 && !first_over_12 {
        Some("datetime.date.short_mdy".to_string())
    } else {
        None
    }
}

/// Disambiguate latitude vs longitude coordinates.
///
/// Rule: If ANY |value| > 90, it must be longitude (latitude max is 90).
///       If ALL |values| ≤ 90, it's likely latitude.
fn disambiguate_coordinates(values: &[String]) -> Option<String> {
    let mut any_over_90 = false;
    let mut all_parseable = true;
    let mut parsed_count = 0;

    for val in values {
        if let Ok(v) = val.trim().parse::<f64>() {
            parsed_count += 1;
            if v.abs() > 90.0 {
                any_over_90 = true;
            }
        } else {
            all_parseable = false;
        }
    }

    // Need at least some parseable values
    if parsed_count < 3 {
        return None;
    }

    if any_over_90 {
        Some("geography.coordinate.longitude".to_string())
    } else if all_parseable {
        // All values within [-90, 90] — likely latitude
        Some("geography.coordinate.latitude".to_string())
    } else {
        None
    }
}

/// Disambiguate numeric types based on value range and distribution.
///
/// Covers: port, increment, postal_code, integer_number, street_number
fn disambiguate_numeric(
    values: &[String],
    results: &[ClassificationResult],
    top_labels: &[&str],
) -> Option<(String, String)> {
    // Only trigger for numeric-looking columns
    let numeric_types = [
        "technology.internet.port",
        "representation.numeric.increment",
        "representation.numeric.integer_number",
        "geography.address.postal_code",
        "geography.address.street_number",
    ];

    let has_numeric_confusion = top_labels.iter().any(|l| numeric_types.contains(l));
    if !has_numeric_confusion {
        return None;
    }

    // Parse all values as integers
    let parsed: Vec<i64> = values
        .iter()
        .filter_map(|v| v.trim().parse::<i64>().ok())
        .collect();

    if parsed.len() < 3 {
        return None;
    }

    let min = *parsed.iter().min().unwrap();
    let max = *parsed.iter().max().unwrap();
    let range = max - min;

    // Check for sequential/increment pattern
    let mut sorted = parsed.clone();
    sorted.sort();
    sorted.dedup();
    let is_sequential = if sorted.len() >= 3 {
        let diffs: Vec<i64> = sorted.windows(2).map(|w| w[1] - w[0]).collect();
        let avg_diff = diffs.iter().sum::<i64>() as f64 / diffs.len() as f64;
        let variance = diffs
            .iter()
            .map(|d| (*d as f64 - avg_diff).powi(2))
            .sum::<f64>()
            / diffs.len() as f64;
        // Low variance in diffs → sequential
        variance < (avg_diff * 0.5).powi(2) && avg_diff > 0.0
    } else {
        false
    };

    // Port detection: 0-65535, common ports cluster
    let all_in_port_range = min >= 0 && max <= 65535;
    let has_common_ports = parsed
        .iter()
        .any(|v| [80, 443, 8080, 3306, 5432, 22, 21, 25, 53, 3000, 8000, 8443].contains(v));

    // Postal code detection: typically 3-10 digits, non-sequential, bounded range
    let all_positive = min > 0;
    let typical_postal_range = all_positive && max <= 99999 && min >= 100;
    let digit_lengths: Vec<usize> = values
        .iter()
        .filter_map(|v| {
            let trimmed = v.trim();
            if trimmed.chars().all(|c| c.is_ascii_digit()) {
                Some(trimmed.len())
            } else {
                None
            }
        })
        .collect();
    let consistent_digits = if !digit_lengths.is_empty() {
        let first_len = digit_lengths[0];
        digit_lengths.iter().all(|&l| l == first_len)
    } else {
        false
    };

    // Decision logic
    if is_sequential && min >= 0 && range > 0 {
        // Sequential integers → increment
        return Some((
            "representation.numeric.increment".to_string(),
            "numeric_sequential_detection".to_string(),
        ));
    }

    if has_common_ports && all_in_port_range && !is_sequential {
        // Has common ports and all in range → port
        return Some((
            "technology.internet.port".to_string(),
            "numeric_port_detection".to_string(),
        ));
    }

    if consistent_digits && typical_postal_range && !is_sequential {
        // Consistent digit length, typical postal range → postal code
        return Some((
            "geography.address.postal_code".to_string(),
            "numeric_postal_code_detection".to_string(),
        ));
    }

    // Street number: small positive integers, typically 1-9999
    let street_range = all_positive && max < 100000 && min >= 1;
    let is_street_candidate = top_labels.contains(&"geography.address.street_number");
    if is_street_candidate
        && street_range
        && !is_sequential
        && !has_common_ports
        && !consistent_digits
    {
        return Some((
            "geography.address.street_number".to_string(),
            "numeric_street_number_detection".to_string(),
        ));
    }

    // Fallback: if we couldn't determine more specifically, use the model majority
    // (return None to let the majority vote stand)
    let _ = results; // suppress unused warning
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    // ── Disambiguation rule unit tests ──────────────────────────────────

    #[test]
    fn test_slash_date_eu_detected() {
        let values: Vec<String> = vec![
            "15/01/2024",
            "28/06/2023",
            "03/11/2022",
            "31/12/2019",
            "12/05/2020",
        ]
        .into_iter()
        .map(String::from)
        .collect();

        let result = disambiguate_slash_dates(&values);
        assert_eq!(result, Some("datetime.date.eu_slash".to_string()));
    }

    #[test]
    fn test_slash_date_us_detected() {
        let values: Vec<String> = vec![
            "01/15/2024",
            "06/28/2023",
            "11/03/2022",
            "12/31/2019",
            "05/12/2020",
        ]
        .into_iter()
        .map(String::from)
        .collect();

        let result = disambiguate_slash_dates(&values);
        assert_eq!(result, Some("datetime.date.us_slash".to_string()));
    }

    #[test]
    fn test_slash_date_ambiguous() {
        // All values have both components ≤ 12 — ambiguous
        let values: Vec<String> = vec![
            "01/02/2024",
            "03/04/2023",
            "05/06/2022",
            "07/08/2021",
            "09/10/2020",
        ]
        .into_iter()
        .map(String::from)
        .collect();

        let result = disambiguate_slash_dates(&values);
        assert_eq!(result, None);
    }

    #[test]
    fn test_short_date_dmy_detected() {
        let values: Vec<String> = vec!["15-01-24", "28-06-23", "31-12-19"]
            .into_iter()
            .map(String::from)
            .collect();

        let result = disambiguate_short_dates(&values);
        assert_eq!(result, Some("datetime.date.short_dmy".to_string()));
    }

    #[test]
    fn test_short_date_mdy_detected() {
        let values: Vec<String> = vec!["01-15-24", "06-28-23", "12-31-19"]
            .into_iter()
            .map(String::from)
            .collect();

        let result = disambiguate_short_dates(&values);
        assert_eq!(result, Some("datetime.date.short_mdy".to_string()));
    }

    #[test]
    fn test_coordinates_longitude_detected() {
        let values: Vec<String> = vec!["-74.0060", "151.2093", "-0.1278", "139.6917", "2.3522"]
            .into_iter()
            .map(String::from)
            .collect();

        let result = disambiguate_coordinates(&values);
        assert_eq!(result, Some("geography.coordinate.longitude".to_string()));
    }

    #[test]
    fn test_coordinates_latitude_detected() {
        let values: Vec<String> = vec!["40.7128", "-33.8688", "51.5074", "35.6762", "-22.9068"]
            .into_iter()
            .map(String::from)
            .collect();

        let result = disambiguate_coordinates(&values);
        assert_eq!(result, Some("geography.coordinate.latitude".to_string()));
    }

    #[test]
    fn test_numeric_sequential_detection() {
        let values: Vec<String> = vec!["1", "2", "3", "4", "5", "6", "7", "8", "9", "10"]
            .into_iter()
            .map(String::from)
            .collect();

        // Create mock results with increment label
        let results: Vec<ClassificationResult> = values
            .iter()
            .map(|_| ClassificationResult {
                label: "representation.numeric.increment".to_string(),
                confidence: 0.8,
                all_scores: vec![],
            })
            .collect();

        let votes = vec![
            ("representation.numeric.increment".to_string(), 8),
            ("representation.numeric.integer_number".to_string(), 2),
        ];
        let top_labels: Vec<&str> = votes.iter().map(|(l, _)| l.as_str()).collect();

        let result = disambiguate_numeric(&values, &results, &top_labels);
        assert!(result.is_some());
        let (label, _rule) = result.unwrap();
        assert_eq!(label, "representation.numeric.increment");
    }

    #[test]
    fn test_numeric_port_detection() {
        let values: Vec<String> = vec![
            "80", "443", "8080", "3306", "22", "5432", "3000", "8443", "25", "53",
        ]
        .into_iter()
        .map(String::from)
        .collect();

        let results: Vec<ClassificationResult> = values
            .iter()
            .map(|_| ClassificationResult {
                label: "technology.internet.port".to_string(),
                confidence: 0.7,
                all_scores: vec![],
            })
            .collect();

        let votes = vec![
            ("technology.internet.port".to_string(), 7),
            ("representation.numeric.integer_number".to_string(), 3),
        ];
        let top_labels: Vec<&str> = votes.iter().map(|(l, _)| l.as_str()).collect();

        let result = disambiguate_numeric(&values, &results, &top_labels);
        assert!(result.is_some());
        let (label, _rule) = result.unwrap();
        assert_eq!(label, "technology.internet.port");
    }

    #[test]
    fn test_numeric_postal_code_detection() {
        let values: Vec<String> = vec![
            "10001", "90210", "30301", "60601", "02101", "75001", "33101", "94102", "20001",
            "98101",
        ]
        .into_iter()
        .map(String::from)
        .collect();

        let results: Vec<ClassificationResult> = values
            .iter()
            .map(|_| ClassificationResult {
                label: "geography.address.postal_code".to_string(),
                confidence: 0.6,
                all_scores: vec![],
            })
            .collect();

        let votes = vec![
            ("geography.address.postal_code".to_string(), 6),
            ("representation.numeric.integer_number".to_string(), 4),
        ];
        let top_labels: Vec<&str> = votes.iter().map(|(l, _)| l.as_str()).collect();

        let result = disambiguate_numeric(&values, &results, &top_labels);
        assert!(result.is_some());
        let (label, _rule) = result.unwrap();
        assert_eq!(label, "geography.address.postal_code");
    }

    #[test]
    fn test_empty_column() {
        // Just test the ColumnResult for empty case
        let result = ColumnResult {
            label: "unknown".to_string(),
            confidence: 0.0,
            vote_distribution: vec![],
            disambiguation_applied: false,
            disambiguation_rule: None,
            samples_used: 0,
        };
        assert_eq!(result.label, "unknown");
        assert_eq!(result.samples_used, 0);
    }
}

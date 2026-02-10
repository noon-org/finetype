//! Validation engine for finetype labels.
//!
//! Validates string values against the JSON Schema fragment stored in each
//! type definition's `validation` field. Supports column-level validation
//! with configurable strategies for handling invalid data.

use crate::taxonomy::{Taxonomy, Validation};
use regex::Regex;
use std::collections::HashMap;
use thiserror::Error;

// ═══════════════════════════════════════════════════════════════════════════════
// ERRORS
// ═══════════════════════════════════════════════════════════════════════════════

#[derive(Error, Debug)]
pub enum ValidatorError {
    #[error("Unknown label: {0}")]
    UnknownLabel(String),
    #[error("No validation schema for label: {0}")]
    NoSchema(String),
    #[error("Invalid regex pattern for label {label}: {source}")]
    InvalidPattern { label: String, source: regex::Error },
}

// ═══════════════════════════════════════════════════════════════════════════════
// VALIDATION RESULT (single value)
// ═══════════════════════════════════════════════════════════════════════════════

/// Result of validating a single value.
#[derive(Debug, Clone)]
pub struct ValidationResult {
    /// Whether the value is valid.
    pub is_valid: bool,
    /// List of validation errors (empty if valid).
    pub errors: Vec<ValidationError>,
}

/// A single validation error with detail.
#[derive(Debug, Clone)]
pub struct ValidationError {
    /// Which check failed.
    pub check: ValidationCheck,
    /// Human-readable message.
    pub message: String,
}

/// The type of validation check that was performed.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ValidationCheck {
    Pattern,
    MinLength,
    MaxLength,
    Minimum,
    Maximum,
    Enum,
}

impl std::fmt::Display for ValidationCheck {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Pattern => write!(f, "pattern"),
            Self::MinLength => write!(f, "minLength"),
            Self::MaxLength => write!(f, "maxLength"),
            Self::Minimum => write!(f, "minimum"),
            Self::Maximum => write!(f, "maximum"),
            Self::Enum => write!(f, "enum"),
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// SINGLE-VALUE VALIDATION
// ═══════════════════════════════════════════════════════════════════════════════

/// Validate a single value against a Validation schema fragment.
///
/// Checks all applicable fields: pattern, minLength, maxLength, minimum, maximum, enum.
/// Returns a ValidationResult with all errors collected.
pub fn validate_value(
    value: &str,
    schema: &Validation,
) -> Result<ValidationResult, ValidatorError> {
    let mut errors = Vec::new();

    // Pattern check
    if let Some(pattern) = &schema.pattern {
        let re = Regex::new(pattern).map_err(|e| ValidatorError::InvalidPattern {
            label: String::new(),
            source: e,
        })?;
        if !re.is_match(value) {
            errors.push(ValidationError {
                check: ValidationCheck::Pattern,
                message: format!("Value does not match pattern: {}", pattern),
            });
        }
    }

    // MinLength check
    if let Some(min_len) = schema.min_length {
        if value.len() < min_len as usize {
            errors.push(ValidationError {
                check: ValidationCheck::MinLength,
                message: format!(
                    "Value length {} is less than minimum {}",
                    value.len(),
                    min_len
                ),
            });
        }
    }

    // MaxLength check
    if let Some(max_len) = schema.max_length {
        if value.len() > max_len as usize {
            errors.push(ValidationError {
                check: ValidationCheck::MaxLength,
                message: format!("Value length {} exceeds maximum {}", value.len(), max_len),
            });
        }
    }

    // Minimum check (parse value as number)
    if let Some(minimum) = schema.minimum {
        if let Ok(num) = value.parse::<f64>() {
            if num < minimum {
                errors.push(ValidationError {
                    check: ValidationCheck::Minimum,
                    message: format!("Value {} is less than minimum {}", num, minimum),
                });
            }
        }
        // If not parseable as number but has minimum constraint, that's not an error
        // (the pattern check should catch format issues)
    }

    // Maximum check (parse value as number)
    if let Some(maximum) = schema.maximum {
        if let Ok(num) = value.parse::<f64>() {
            if num > maximum {
                errors.push(ValidationError {
                    check: ValidationCheck::Maximum,
                    message: format!("Value {} exceeds maximum {}", num, maximum),
                });
            }
        }
    }

    // Enum check
    if let Some(enum_values) = &schema.enum_values {
        if !enum_values.iter().any(|v| v == value) {
            errors.push(ValidationError {
                check: ValidationCheck::Enum,
                message: format!(
                    "Value '{}' is not in allowed values: {:?}",
                    value, enum_values
                ),
            });
        }
    }

    Ok(ValidationResult {
        is_valid: errors.is_empty(),
        errors,
    })
}

/// Validate a value against a label's schema from the taxonomy.
///
/// Convenience function that looks up the schema from the taxonomy.
pub fn validate_value_for_label(
    value: &str,
    label: &str,
    taxonomy: &Taxonomy,
) -> Result<ValidationResult, ValidatorError> {
    let definition = taxonomy
        .get(label)
        .ok_or_else(|| ValidatorError::UnknownLabel(label.to_string()))?;

    let schema = definition
        .validation
        .as_ref()
        .ok_or_else(|| ValidatorError::NoSchema(label.to_string()))?;

    validate_value(value, schema)
}

// ═══════════════════════════════════════════════════════════════════════════════
// COLUMN VALIDATION WITH STRATEGIES
// ═══════════════════════════════════════════════════════════════════════════════

/// Strategy for handling invalid values during column validation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum InvalidStrategy {
    /// Collect invalid values separately for review (default).
    #[default]
    Quarantine,
    /// Replace invalid values with NULL.
    SetNull,
    /// Replace invalid values with the last valid value.
    ForwardFill,
    /// Replace invalid values with the next valid value.
    BackwardFill,
}

/// A quarantined invalid value with context.
#[derive(Debug, Clone)]
pub struct QuarantinedValue {
    /// Row index (0-based).
    pub row_index: usize,
    /// The original value.
    pub value: String,
    /// Validation errors for this value.
    pub errors: Vec<ValidationError>,
}

/// Statistics from column validation.
#[derive(Debug, Clone)]
pub struct ColumnStats {
    /// Number of valid values.
    pub valid_count: usize,
    /// Number of invalid values.
    pub invalid_count: usize,
    /// Number of NULL values.
    pub null_count: usize,
    /// Total number of values.
    pub total_count: usize,
    /// Error pattern summary: check type → count of failures.
    pub error_patterns: HashMap<ValidationCheck, usize>,
}

impl ColumnStats {
    /// Percentage of valid (non-null) values.
    pub fn validity_rate(&self) -> f64 {
        let non_null = self.total_count - self.null_count;
        if non_null == 0 {
            return 0.0;
        }
        self.valid_count as f64 / non_null as f64
    }
}

/// Result of validating a column of values.
#[derive(Debug, Clone)]
pub struct ColumnValidationResult {
    /// The output values after applying the strategy.
    /// None represents NULL.
    pub values: Vec<Option<String>>,
    /// Validation statistics.
    pub stats: ColumnStats,
    /// Quarantined values (only populated in Quarantine mode).
    pub quarantined: Vec<QuarantinedValue>,
}

/// Validate a column of values against a schema with a specified strategy.
///
/// Each value is `Option<&str>` where None represents NULL.
pub fn validate_column(
    values: &[Option<&str>],
    schema: &Validation,
    strategy: InvalidStrategy,
) -> Result<ColumnValidationResult, ValidatorError> {
    let total_count = values.len();
    let mut valid_count = 0;
    let mut invalid_count = 0;
    let mut null_count = 0;
    let mut error_patterns: HashMap<ValidationCheck, usize> = HashMap::new();
    let mut quarantined: Vec<QuarantinedValue> = Vec::new();

    // First pass: validate all values and collect results
    let mut validation_results: Vec<Option<ValidationResult>> = Vec::with_capacity(total_count);
    for value in values {
        match value {
            None => {
                null_count += 1;
                validation_results.push(None);
            }
            Some(v) => {
                let result = validate_value(v, schema)?;
                if result.is_valid {
                    valid_count += 1;
                } else {
                    invalid_count += 1;
                    for error in &result.errors {
                        *error_patterns.entry(error.check.clone()).or_insert(0) += 1;
                    }
                }
                validation_results.push(Some(result));
            }
        }
    }

    // Second pass: apply strategy to produce output values
    let output_values = match strategy {
        InvalidStrategy::Quarantine => {
            let mut output = Vec::with_capacity(total_count);
            for (i, (value, result)) in values.iter().zip(validation_results.iter()).enumerate() {
                match (value, result) {
                    (None, _) => output.push(None),
                    (Some(v), Some(r)) if !r.is_valid => {
                        quarantined.push(QuarantinedValue {
                            row_index: i,
                            value: v.to_string(),
                            errors: r.errors.clone(),
                        });
                        // In quarantine mode, invalid values are removed (set to None)
                        output.push(None);
                    }
                    (Some(v), _) => output.push(Some(v.to_string())),
                }
            }
            output
        }
        InvalidStrategy::SetNull => {
            let mut output = Vec::with_capacity(total_count);
            for (value, result) in values.iter().zip(validation_results.iter()) {
                match (value, result) {
                    (None, _) => output.push(None),
                    (Some(_), Some(r)) if !r.is_valid => output.push(None),
                    (Some(v), _) => output.push(Some(v.to_string())),
                }
            }
            output
        }
        InvalidStrategy::ForwardFill => {
            let mut output = Vec::with_capacity(total_count);
            let mut last_valid: Option<String> = None;
            for (value, result) in values.iter().zip(validation_results.iter()) {
                match (value, result) {
                    (None, _) => output.push(None), // NULLs stay NULL
                    (Some(v), Some(r)) if r.is_valid => {
                        last_valid = Some(v.to_string());
                        output.push(Some(v.to_string()));
                    }
                    (Some(_), Some(_)) => {
                        // Invalid: use last valid value
                        output.push(last_valid.clone());
                    }
                    (Some(v), None) => {
                        // Shouldn't happen (non-null values always get validated)
                        output.push(Some(v.to_string()));
                    }
                }
            }
            output
        }
        InvalidStrategy::BackwardFill => {
            // First, find the next valid value for each position
            let mut output: Vec<Option<String>> = vec![None; total_count];
            let mut next_valid: Option<String> = None;

            // Reverse pass to find next valid
            for i in (0..total_count).rev() {
                match (&values[i], &validation_results[i]) {
                    (None, _) => {
                        output[i] = None; // NULLs stay NULL
                    }
                    (Some(v), Some(r)) if r.is_valid => {
                        next_valid = Some(v.to_string());
                        output[i] = Some(v.to_string());
                    }
                    (Some(_), Some(_)) => {
                        // Invalid: use next valid value
                        output[i] = next_valid.clone();
                    }
                    (Some(v), None) => {
                        output[i] = Some(v.to_string());
                    }
                }
            }
            output
        }
    };

    Ok(ColumnValidationResult {
        values: output_values,
        stats: ColumnStats {
            valid_count,
            invalid_count,
            null_count,
            total_count,
            error_patterns,
        },
        quarantined,
    })
}

/// Validate a column of values against a label's schema from the taxonomy.
pub fn validate_column_for_label(
    values: &[Option<&str>],
    label: &str,
    taxonomy: &Taxonomy,
    strategy: InvalidStrategy,
) -> Result<ColumnValidationResult, ValidatorError> {
    let definition = taxonomy
        .get(label)
        .ok_or_else(|| ValidatorError::UnknownLabel(label.to_string()))?;

    let schema = definition
        .validation
        .as_ref()
        .ok_or_else(|| ValidatorError::NoSchema(label.to_string()))?;

    validate_column(values, schema, strategy)
}

// ═══════════════════════════════════════════════════════════════════════════════
// TESTS
// ═══════════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;

    fn ip_schema() -> Validation {
        Validation {
            schema_type: Some("string".to_string()),
            pattern: Some(
                r"^(?:(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.){3}(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)$"
                    .to_string(),
            ),
            min_length: Some(7),
            max_length: Some(15),
            minimum: None,
            maximum: None,
            enum_values: None,
        }
    }

    fn boolean_schema() -> Validation {
        Validation {
            schema_type: Some("string".to_string()),
            pattern: None,
            min_length: None,
            max_length: None,
            minimum: None,
            maximum: None,
            enum_values: Some(vec![
                "true".to_string(),
                "false".to_string(),
                "True".to_string(),
                "False".to_string(),
                "TRUE".to_string(),
                "FALSE".to_string(),
                "yes".to_string(),
                "no".to_string(),
                "0".to_string(),
                "1".to_string(),
            ]),
        }
    }

    fn port_schema() -> Validation {
        Validation {
            schema_type: Some("string".to_string()),
            pattern: Some(r"^\d+$".to_string()),
            min_length: None,
            max_length: None,
            minimum: Some(0.0),
            maximum: Some(65535.0),
            enum_values: None,
        }
    }

    // ── Single-value validation tests ────────────────────────────────────

    #[test]
    fn test_valid_ipv4() {
        let result = validate_value("192.168.1.1", &ip_schema()).unwrap();
        assert!(result.is_valid);
        assert!(result.errors.is_empty());
    }

    #[test]
    fn test_invalid_ipv4_pattern() {
        let result = validate_value("999.999.999.999", &ip_schema()).unwrap();
        assert!(!result.is_valid);
        assert_eq!(result.errors.len(), 1);
        assert_eq!(result.errors[0].check, ValidationCheck::Pattern);
    }

    #[test]
    fn test_invalid_ipv4_too_short() {
        let result = validate_value("1.1.1", &ip_schema()).unwrap();
        assert!(!result.is_valid);
        // Should fail both pattern and minLength
        let checks: Vec<&ValidationCheck> = result.errors.iter().map(|e| &e.check).collect();
        assert!(checks.contains(&&ValidationCheck::Pattern));
        assert!(checks.contains(&&ValidationCheck::MinLength));
    }

    #[test]
    fn test_valid_boolean_enum() {
        let result = validate_value("true", &boolean_schema()).unwrap();
        assert!(result.is_valid);
    }

    #[test]
    fn test_invalid_boolean_enum() {
        let result = validate_value("maybe", &boolean_schema()).unwrap();
        assert!(!result.is_valid);
        assert_eq!(result.errors[0].check, ValidationCheck::Enum);
    }

    #[test]
    fn test_valid_port() {
        let result = validate_value("8080", &port_schema()).unwrap();
        assert!(result.is_valid);
    }

    #[test]
    fn test_invalid_port_too_high() {
        let result = validate_value("70000", &port_schema()).unwrap();
        assert!(!result.is_valid);
        assert_eq!(result.errors[0].check, ValidationCheck::Maximum);
    }

    #[test]
    fn test_no_constraints_always_valid() {
        let empty_schema = Validation {
            schema_type: Some("string".to_string()),
            pattern: None,
            min_length: None,
            max_length: None,
            minimum: None,
            maximum: None,
            enum_values: None,
        };
        let result = validate_value("anything", &empty_schema).unwrap();
        assert!(result.is_valid);
    }

    // ── Column validation tests ──────────────────────────────────────────

    #[test]
    fn test_column_quarantine() {
        let values = vec![
            Some("192.168.1.1"),
            Some("10.0.0.1"),
            Some("not-an-ip"),
            None,
            Some("172.16.0.1"),
        ];
        let result = validate_column(&values, &ip_schema(), InvalidStrategy::Quarantine).unwrap();

        assert_eq!(result.stats.valid_count, 3);
        assert_eq!(result.stats.invalid_count, 1);
        assert_eq!(result.stats.null_count, 1);
        assert_eq!(result.stats.total_count, 5);
        assert_eq!(result.quarantined.len(), 1);
        assert_eq!(result.quarantined[0].row_index, 2);
        assert_eq!(result.quarantined[0].value, "not-an-ip");

        // Invalid row becomes None in output
        assert_eq!(result.values[2], None);
        // Valid rows preserved
        assert_eq!(result.values[0], Some("192.168.1.1".to_string()));
    }

    #[test]
    fn test_column_set_null() {
        let values = vec![Some("192.168.1.1"), Some("not-an-ip"), Some("10.0.0.1")];
        let result = validate_column(&values, &ip_schema(), InvalidStrategy::SetNull).unwrap();

        assert_eq!(result.stats.valid_count, 2);
        assert_eq!(result.stats.invalid_count, 1);
        assert_eq!(result.values[0], Some("192.168.1.1".to_string()));
        assert_eq!(result.values[1], None);
        assert_eq!(result.values[2], Some("10.0.0.1".to_string()));
    }

    #[test]
    fn test_column_forward_fill() {
        let values = vec![
            Some("192.168.1.1"),
            Some("not-an-ip"),
            Some("10.0.0.1"),
            Some("bad"),
        ];
        let result = validate_column(&values, &ip_schema(), InvalidStrategy::ForwardFill).unwrap();

        assert_eq!(result.values[0], Some("192.168.1.1".to_string()));
        assert_eq!(result.values[1], Some("192.168.1.1".to_string())); // ffill from [0]
        assert_eq!(result.values[2], Some("10.0.0.1".to_string()));
        assert_eq!(result.values[3], Some("10.0.0.1".to_string())); // ffill from [2]
    }

    #[test]
    fn test_column_backward_fill() {
        let values = vec![
            Some("not-an-ip"),
            Some("192.168.1.1"),
            Some("bad"),
            Some("10.0.0.1"),
        ];
        let result = validate_column(&values, &ip_schema(), InvalidStrategy::BackwardFill).unwrap();

        assert_eq!(result.values[0], Some("192.168.1.1".to_string())); // bfill from [1]
        assert_eq!(result.values[1], Some("192.168.1.1".to_string()));
        assert_eq!(result.values[2], Some("10.0.0.1".to_string())); // bfill from [3]
        assert_eq!(result.values[3], Some("10.0.0.1".to_string()));
    }

    #[test]
    fn test_column_ffill_no_prior_valid() {
        let values = vec![Some("bad"), Some("192.168.1.1")];
        let result = validate_column(&values, &ip_schema(), InvalidStrategy::ForwardFill).unwrap();

        // No prior valid value → None
        assert_eq!(result.values[0], None);
        assert_eq!(result.values[1], Some("192.168.1.1".to_string()));
    }

    #[test]
    fn test_column_bfill_no_next_valid() {
        let values = vec![Some("192.168.1.1"), Some("bad")];
        let result = validate_column(&values, &ip_schema(), InvalidStrategy::BackwardFill).unwrap();

        assert_eq!(result.values[0], Some("192.168.1.1".to_string()));
        // No next valid value → None
        assert_eq!(result.values[1], None);
    }

    #[test]
    fn test_column_stats_error_patterns() {
        let values = vec![
            Some("192.168.1.1"),
            Some("x"),      // fails pattern + minLength
            Some("not-ip"), // fails pattern + minLength
        ];
        let result = validate_column(&values, &ip_schema(), InvalidStrategy::Quarantine).unwrap();

        assert_eq!(result.stats.valid_count, 1);
        assert_eq!(result.stats.invalid_count, 2);
        assert_eq!(
            result.stats.error_patterns.get(&ValidationCheck::Pattern),
            Some(&2)
        );
        // Both "x" (len=1) and "not-ip" (len=6) are < minLength 7
        assert_eq!(
            result.stats.error_patterns.get(&ValidationCheck::MinLength),
            Some(&2)
        );
    }

    #[test]
    fn test_column_all_nulls() {
        let values: Vec<Option<&str>> = vec![None, None, None];
        let result = validate_column(&values, &ip_schema(), InvalidStrategy::Quarantine).unwrap();

        assert_eq!(result.stats.null_count, 3);
        assert_eq!(result.stats.valid_count, 0);
        assert_eq!(result.stats.invalid_count, 0);
    }

    #[test]
    fn test_validity_rate() {
        let stats = ColumnStats {
            valid_count: 8,
            invalid_count: 2,
            null_count: 5,
            total_count: 15,
            error_patterns: HashMap::new(),
        };
        // 8 valid out of 10 non-null = 80%
        assert!((stats.validity_rate() - 0.8).abs() < f64::EPSILON);
    }

    #[test]
    fn test_validity_rate_all_null() {
        let stats = ColumnStats {
            valid_count: 0,
            invalid_count: 0,
            null_count: 5,
            total_count: 5,
            error_patterns: HashMap::new(),
        };
        assert!((stats.validity_rate() - 0.0).abs() < f64::EPSILON);
    }

    // ── Taxonomy integration test ────────────────────────────────────────

    #[test]
    fn test_validate_with_taxonomy() {
        let yaml = r#"
datetime.timestamp.iso_8601:
  title: "ISO 8601"
  broad_type: TIMESTAMP
  validation:
    type: string
    pattern: "^\\d{4}-\\d{2}-\\d{2}T\\d{2}:\\d{2}:\\d{2}Z$"
    minLength: 20
    maxLength: 20
  tier: [TIMESTAMP, timestamp]
  release_priority: 5
  samples: ["2024-01-15T10:30:00Z"]
"#;
        let taxonomy = Taxonomy::from_yaml(yaml).unwrap();

        let result = validate_value_for_label(
            "2024-01-15T10:30:00Z",
            "datetime.timestamp.iso_8601",
            &taxonomy,
        )
        .unwrap();
        assert!(result.is_valid);

        let result =
            validate_value_for_label("not-a-timestamp", "datetime.timestamp.iso_8601", &taxonomy)
                .unwrap();
        assert!(!result.is_valid);
    }

    #[test]
    fn test_unknown_label_error() {
        let yaml = r#"
datetime.timestamp.iso_8601:
  title: "ISO 8601"
  validation:
    type: string
  tier: [TIMESTAMP, timestamp]
  samples: ["2024-01-15T10:30:00Z"]
"#;
        let taxonomy = Taxonomy::from_yaml(yaml).unwrap();
        let result = validate_value_for_label("test", "nonexistent.label", &taxonomy);
        assert!(matches!(result, Err(ValidatorError::UnknownLabel(_))));
    }
}

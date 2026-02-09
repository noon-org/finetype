//! Taxonomy definitions for FineType labels.
//!
//! The taxonomy is organized hierarchically:
//! - Domain (e.g., `datetime`, `technology`, `identity`)
//! - Category (e.g., `timestamp`, `internet`, `person`)
//! - Type (e.g., `iso_8601`, `ip_v4`, `email`)
//! - Full label: `domain.category.type.LOCALE`
//!
//! Each definition is a transformation contract — not just a label.
//! If the model says `datetime.date.us_slash`, that is a contract that
//! `strptime(value, '%m/%d/%Y')::DATE` will succeed.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;
use thiserror::Error;

/// Errors that can occur when working with the taxonomy.
#[derive(Error, Debug)]
pub enum TaxonomyError {
    #[error("Failed to read taxonomy file: {0}")]
    IoError(#[from] std::io::Error),
    #[error("Failed to parse taxonomy YAML: {0}")]
    ParseError(#[from] serde_yaml::Error),
    #[error("Invalid label key (expected domain.category.type): {0}")]
    InvalidKey(String),
    #[error("No definition files found in: {0}")]
    NoFiles(String),
    #[error("Glob pattern error: {0}")]
    GlobError(String),
}

/// Designation indicates the scope and stability of a label.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Designation {
    /// Universal format, works across all locales
    Universal,
    /// Locale-specific format
    LocaleSpecific,
    /// Broad category - numbers
    BroadNumbers,
    /// Broad category - characters/strings
    BroadCharacters,
    /// Broad category - words/text
    BroadWords,
    /// Broad category - objects/structured data
    BroadObject,
}

impl Default for Designation {
    fn default() -> Self {
        Designation::Universal
    }
}

/// JSON Schema validation fragment.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Validation {
    #[serde(rename = "type")]
    pub schema_type: Option<String>,
    pub pattern: Option<String>,
    #[serde(rename = "minLength")]
    pub min_length: Option<u32>,
    #[serde(rename = "maxLength")]
    pub max_length: Option<u32>,
    pub minimum: Option<f64>,
    pub maximum: Option<f64>,
    #[serde(rename = "enum")]
    pub enum_values: Option<Vec<String>>,
}

/// A single label definition in the taxonomy.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Definition {
    /// Human-readable title
    pub title: Option<String>,
    /// Description of the label
    pub description: Option<String>,
    /// Designation/scope of the label
    #[serde(default)]
    pub designation: Designation,
    /// Supported locales
    #[serde(default)]
    pub locales: Vec<String>,
    /// Target DuckDB type
    pub broad_type: Option<String>,
    /// DuckDB strptime format string (null if not strptime-based)
    pub format_string: Option<String>,
    /// DuckDB SQL expression ({col} = column placeholder)
    pub transform: Option<String>,
    /// Enhanced transform requiring a DuckDB extension
    pub transform_ext: Option<String>,
    /// Struct expansion for multi-field output
    #[serde(default)]
    pub decompose: Option<serde_yaml::Value>,
    /// JSON Schema fragment for data quality checks
    pub validation: Option<Validation>,
    /// Path from root to parent in the inference graph
    #[serde(default)]
    pub tier: Vec<String>,
    /// Release priority (higher = more important)
    #[serde(default)]
    pub release_priority: u8,
    /// Aliases for this label
    pub aliases: Option<Vec<String>>,
    /// Example samples
    #[serde(default)]
    pub samples: Vec<serde_yaml::Value>,
    /// External references
    pub references: Option<serde_yaml::Value>,
    /// Notes about the label
    pub notes: Option<String>,
}

/// Parsed label with domain, category, and type components.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Label {
    pub domain: String,
    pub category: String,
    pub type_name: String,
}

impl Label {
    /// Parse a label key like "datetime.timestamp.iso_8601"
    pub fn parse(s: &str) -> Option<Self> {
        let parts: Vec<&str> = s.split('.').collect();
        if parts.len() == 3 {
            Some(Label {
                domain: parts[0].to_string(),
                category: parts[1].to_string(),
                type_name: parts[2].to_string(),
            })
        } else {
            None
        }
    }

    /// Get the full key (domain.category.type)
    pub fn key(&self) -> String {
        format!("{}.{}.{}", self.domain, self.category, self.type_name)
    }

    /// Get the full label with locale
    pub fn with_locale(&self, locale: &str) -> String {
        format!("{}.{}.{}.{}", self.domain, self.category, self.type_name, locale)
    }
}

impl std::fmt::Display for Label {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.key())
    }
}

/// The complete taxonomy of label definitions.
#[derive(Debug, Clone)]
pub struct Taxonomy {
    definitions: HashMap<String, Definition>,
    labels: Vec<String>,
}

impl Taxonomy {
    /// Load taxonomy from a single YAML file.
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, TaxonomyError> {
        let content = std::fs::read_to_string(path)?;
        Self::from_yaml(&content)
    }

    /// Load taxonomy from all definitions_*.yaml files in a directory.
    pub fn from_directory<P: AsRef<Path>>(dir: P) -> Result<Self, TaxonomyError> {
        let pattern = dir.as_ref().join("definitions_*.yaml");
        let pattern_str = pattern.to_string_lossy().to_string();

        let paths: Vec<_> = glob::glob(&pattern_str)
            .map_err(|e| TaxonomyError::GlobError(e.to_string()))?
            .filter_map(|entry| entry.ok())
            .collect();

        if paths.is_empty() {
            return Err(TaxonomyError::NoFiles(pattern_str));
        }

        let mut all_definitions = HashMap::new();

        for path in paths {
            let content = std::fs::read_to_string(&path)?;
            let defs: HashMap<String, Definition> = serde_yaml::from_str(&content)?;
            all_definitions.extend(defs);
        }

        let mut labels: Vec<String> = all_definitions.keys().cloned().collect();
        labels.sort();

        Ok(Taxonomy {
            definitions: all_definitions,
            labels,
        })
    }

    /// Parse taxonomy from YAML string.
    pub fn from_yaml(yaml: &str) -> Result<Self, TaxonomyError> {
        let raw: HashMap<String, Definition> = serde_yaml::from_str(yaml)?;

        let mut labels: Vec<String> = raw.keys().cloned().collect();
        labels.sort();

        Ok(Taxonomy {
            definitions: raw,
            labels,
        })
    }

    /// Get a definition by its full key (e.g., "datetime.timestamp.iso_8601")
    pub fn get(&self, key: &str) -> Option<&Definition> {
        self.definitions.get(key)
    }

    /// Get all label keys (sorted)
    pub fn labels(&self) -> &[String] {
        &self.labels
    }

    /// Get all definitions
    pub fn definitions(&self) -> impl Iterator<Item = (&String, &Definition)> {
        self.definitions.iter()
    }

    /// Get definitions at or above a priority level
    pub fn at_priority(&self, min_priority: u8) -> Vec<(&String, &Definition)> {
        self.definitions
            .iter()
            .filter(|(_, d)| d.release_priority >= min_priority)
            .collect()
    }

    /// Get definitions by domain
    pub fn by_domain(&self, domain: &str) -> Vec<(&String, &Definition)> {
        self.definitions
            .iter()
            .filter(|(k, _)| k.starts_with(&format!("{}.", domain)))
            .collect()
    }

    /// Get definitions by domain and category
    pub fn by_category(&self, domain: &str, category: &str) -> Vec<(&String, &Definition)> {
        let prefix = format!("{}.{}.", domain, category);
        self.definitions
            .iter()
            .filter(|(k, _)| k.starts_with(&prefix))
            .collect()
    }

    /// Get all unique domains
    pub fn domains(&self) -> Vec<String> {
        let mut domains: Vec<String> = self
            .definitions
            .keys()
            .filter_map(|k| k.split('.').next().map(String::from))
            .collect();
        domains.sort();
        domains.dedup();
        domains
    }

    /// Get all unique categories within a domain
    pub fn categories(&self, domain: &str) -> Vec<String> {
        let prefix = format!("{}.", domain);
        let mut cats: Vec<String> = self
            .definitions
            .keys()
            .filter(|k| k.starts_with(&prefix))
            .filter_map(|k| k.split('.').nth(1).map(String::from))
            .collect();
        cats.sort();
        cats.dedup();
        cats
    }

    /// Number of definitions
    pub fn len(&self) -> usize {
        self.definitions.len()
    }

    /// Check if taxonomy is empty
    pub fn is_empty(&self) -> bool {
        self.definitions.is_empty()
    }

    /// Create label to index mapping for model training
    pub fn label_to_index(&self) -> HashMap<String, usize> {
        self.labels
            .iter()
            .enumerate()
            .map(|(i, l)| (l.clone(), i))
            .collect()
    }

    /// Create index to label mapping for model inference
    pub fn index_to_label(&self) -> HashMap<usize, String> {
        self.labels
            .iter()
            .enumerate()
            .map(|(i, l)| (i, l.clone()))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_YAML: &str = r#"
datetime.timestamp.iso_8601:
  title: "ISO 8601"
  description: "Standard international datetime format"
  designation: universal
  locales: [UNIVERSAL]
  broad_type: TIMESTAMP
  format_string: "%Y-%m-%dT%H:%M:%SZ"
  transform: "strptime({col}, '%Y-%m-%dT%H:%M:%SZ')"
  transform_ext: null
  decompose: null
  validation:
    type: string
    pattern: "^\\d{4}-\\d{2}-\\d{2}T\\d{2}:\\d{2}:\\d{2}Z$"
    minLength: 20
    maxLength: 20
  tier: [TIMESTAMP, timestamp]
  release_priority: 5
  aliases: [big_endian]
  samples:
    - "2024-01-15T10:30:00Z"
  references: null
  notes: null
"#;

    #[test]
    fn test_parse_yaml() {
        let taxonomy = Taxonomy::from_yaml(SAMPLE_YAML).unwrap();
        assert_eq!(taxonomy.len(), 1);
        assert_eq!(taxonomy.labels(), &["datetime.timestamp.iso_8601"]);
    }

    #[test]
    fn test_label_parse() {
        let label = Label::parse("datetime.timestamp.iso_8601").unwrap();
        assert_eq!(label.domain, "datetime");
        assert_eq!(label.category, "timestamp");
        assert_eq!(label.type_name, "iso_8601");
        assert_eq!(label.key(), "datetime.timestamp.iso_8601");
    }

    #[test]
    fn test_label_with_locale() {
        let label = Label::parse("datetime.date.abbreviated_month").unwrap();
        assert_eq!(
            label.with_locale("FR"),
            "datetime.date.abbreviated_month.FR"
        );
    }

    #[test]
    fn test_get_definition() {
        let taxonomy = Taxonomy::from_yaml(SAMPLE_YAML).unwrap();
        let def = taxonomy.get("datetime.timestamp.iso_8601").unwrap();
        assert_eq!(def.title.as_deref(), Some("ISO 8601"));
        assert_eq!(def.broad_type.as_deref(), Some("TIMESTAMP"));
        assert_eq!(def.release_priority, 5);
    }

    #[test]
    fn test_domains() {
        let taxonomy = Taxonomy::from_yaml(SAMPLE_YAML).unwrap();
        assert_eq!(taxonomy.domains(), vec!["datetime"]);
    }

    #[test]
    fn test_categories() {
        let taxonomy = Taxonomy::from_yaml(SAMPLE_YAML).unwrap();
        assert_eq!(taxonomy.categories("datetime"), vec!["timestamp"]);
    }

    #[test]
    fn test_at_priority() {
        let taxonomy = Taxonomy::from_yaml(SAMPLE_YAML).unwrap();
        assert_eq!(taxonomy.at_priority(5).len(), 1);
        assert_eq!(taxonomy.at_priority(6).len(), 0);
    }
}

//! Taxonomy definitions for FineType labels.
//!
//! The taxonomy is organized hierarchically:
//! - Provider (e.g., `address`, `datetime`, `internet`)
//! - Method (e.g., `ip_v4`, `email`, `iso_8601`)
//! - Full label: `provider.method`

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
    #[error("Unknown label: {0}")]
    UnknownLabel(String),
}

/// Locale identifier for locale-specific labels.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Locale {
    Universal,
    En,
    EnAu,
    EnCa,
    EnGb,
    De,
    DeAt,
    DeCh,
    Fr,
    Es,
    EsMx,
    It,
    Pt,
    PtBr,
    Nl,
    NlBe,
    Ja,
    Ko,
    Zh,
    Ru,
    Pl,
    Cs,
    Da,
    Fi,
    Sv,
    No,
    Hu,
    Hr,
    Sk,
    Et,
    El,
    Tr,
    Uk,
    Fa,
    Is,
    Kk,
    #[serde(rename = "AR_AE")]
    ArAe,
    #[serde(rename = "AR_DZ")]
    ArDz,
    #[serde(rename = "AR_EG")]
    ArEg,
    #[serde(rename = "AR_JO")]
    ArJo,
    #[serde(rename = "AR_OM")]
    ArOm,
    #[serde(rename = "AR_SY")]
    ArSy,
    #[serde(rename = "AR_YE")]
    ArYe,
    #[serde(other)]
    Other,
}

impl Default for Locale {
    fn default() -> Self {
        Locale::Universal
    }
}

impl std::fmt::Display for Locale {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Locale::Universal => write!(f, "UNIVERSAL"),
            Locale::En => write!(f, "EN"),
            Locale::EnAu => write!(f, "EN_AU"),
            Locale::EnCa => write!(f, "EN_CA"),
            Locale::EnGb => write!(f, "EN_GB"),
            Locale::De => write!(f, "DE"),
            Locale::DeAt => write!(f, "DE_AT"),
            Locale::DeCh => write!(f, "DE_CH"),
            Locale::Fr => write!(f, "FR"),
            Locale::Es => write!(f, "ES"),
            Locale::EsMx => write!(f, "ES_MX"),
            Locale::It => write!(f, "IT"),
            Locale::Pt => write!(f, "PT"),
            Locale::PtBr => write!(f, "PT_BR"),
            Locale::Nl => write!(f, "NL"),
            Locale::NlBe => write!(f, "NL_BE"),
            Locale::Ja => write!(f, "JA"),
            Locale::Ko => write!(f, "KO"),
            Locale::Zh => write!(f, "ZH"),
            Locale::Ru => write!(f, "RU"),
            Locale::Pl => write!(f, "PL"),
            Locale::Cs => write!(f, "CS"),
            Locale::Da => write!(f, "DA"),
            Locale::Fi => write!(f, "FI"),
            Locale::Sv => write!(f, "SV"),
            Locale::No => write!(f, "NO"),
            Locale::Hu => write!(f, "HU"),
            Locale::Hr => write!(f, "HR"),
            Locale::Sk => write!(f, "SK"),
            Locale::Et => write!(f, "ET"),
            Locale::El => write!(f, "EL"),
            Locale::Tr => write!(f, "TR"),
            Locale::Uk => write!(f, "UK"),
            Locale::Fa => write!(f, "FA"),
            Locale::Is => write!(f, "IS"),
            Locale::Kk => write!(f, "KK"),
            Locale::ArAe => write!(f, "AR_AE"),
            Locale::ArDz => write!(f, "AR_DZ"),
            Locale::ArEg => write!(f, "AR_EG"),
            Locale::ArJo => write!(f, "AR_JO"),
            Locale::ArOm => write!(f, "AR_OM"),
            Locale::ArSy => write!(f, "AR_SY"),
            Locale::ArYe => write!(f, "AR_YE"),
            Locale::Other => write!(f, "OTHER"),
        }
    }
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
    /// Duplicate of another label
    Duplicate,
}

impl Default for Designation {
    fn default() -> Self {
        Designation::Universal
    }
}

/// Reference link for a definition.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Reference {
    pub title: String,
    pub link: String,
}

/// A single label definition in the taxonomy.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Definition {
    /// Human-readable title
    pub title: Option<String>,
    /// Description of the label
    pub description: Option<String>,
    /// Aliases for this label
    pub aliases: Option<Vec<String>>,
    /// Designation/scope of the label
    #[serde(default)]
    pub designation: Designation,
    /// Supported locales
    #[serde(default)]
    pub locales: Vec<Locale>,
    /// Provider name (e.g., "address", "datetime")
    pub provider: String,
    /// Method name (e.g., "ip_v4", "iso_8601")
    pub method: String,
    /// Notes about the label
    pub notes: Option<String>,
    /// Primitive type (str, int, float, etc.)
    pub primitive: Option<String>,
    /// External references
    pub references: Option<Vec<Reference>>,
    /// Release priority (higher = more important)
    #[serde(default)]
    pub release_priority: u8,
    /// Example samples
    #[serde(default)]
    pub samples: Vec<serde_yaml::Value>,
}

impl Definition {
    /// Get the full label name (provider.method)
    pub fn label(&self) -> String {
        format!("{}.{}", self.provider, self.method)
    }

    /// Check if this definition should be included at a given priority level
    pub fn included_at_priority(&self, min_priority: u8) -> bool {
        self.release_priority >= min_priority
    }

    /// Check if this is a universal (locale-independent) definition
    pub fn is_universal(&self) -> bool {
        self.locales.len() == 1 && self.locales[0] == Locale::Universal
    }
}

/// A parsed label with provider and method components.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Label {
    pub provider: String,
    pub method: String,
    pub locale: Option<Locale>,
}

impl Label {
    /// Parse a label string like "datetime.iso_8601" or "datetime.iso_8601.EN"
    pub fn parse(s: &str) -> Option<Self> {
        let parts: Vec<&str> = s.split('.').collect();
        match parts.len() {
            2 => Some(Label {
                provider: parts[0].to_string(),
                method: parts[1].to_string(),
                locale: None,
            }),
            3 => {
                // Parse locale (simplified for now)
                Some(Label {
                    provider: parts[0].to_string(),
                    method: parts[1].to_string(),
                    locale: Some(Locale::Universal),
                })
            }
            _ => None,
        }
    }

    /// Get the base label without locale
    pub fn base(&self) -> String {
        format!("{}.{}", self.provider, self.method)
    }

    /// Get the full label with locale if present
    pub fn full(&self) -> String {
        match &self.locale {
            Some(loc) => format!("{}.{}.{}", self.provider, self.method, loc),
            None => self.base(),
        }
    }
}

impl std::fmt::Display for Label {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.full())
    }
}

/// The complete taxonomy of label definitions.
#[derive(Debug, Clone)]
pub struct Taxonomy {
    definitions: HashMap<String, Definition>,
    labels: Vec<String>,
}

impl Taxonomy {
    /// Load taxonomy from a YAML file.
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, TaxonomyError> {
        let content = std::fs::read_to_string(path)?;
        Self::from_yaml(&content)
    }

    /// Parse taxonomy from YAML string.
    pub fn from_yaml(yaml: &str) -> Result<Self, TaxonomyError> {
        let raw: HashMap<String, Definition> = serde_yaml::from_str(yaml)?;
        
        // Sort labels for deterministic ordering across training and inference
        let mut labels: Vec<String> = raw.keys().cloned().collect();
        labels.sort();
        
        Ok(Taxonomy {
            definitions: raw,
            labels,
        })
    }

    /// Get a definition by its full label (e.g., "datetime.iso_8601")
    pub fn get(&self, label: &str) -> Option<&Definition> {
        self.definitions.get(label)
    }

    /// Get all labels
    pub fn labels(&self) -> &[String] {
        &self.labels
    }

    /// Get all definitions
    pub fn definitions(&self) -> impl Iterator<Item = (&String, &Definition)> {
        self.definitions.iter()
    }

    /// Get definitions at or above a priority level
    pub fn at_priority(&self, min_priority: u8) -> Vec<&Definition> {
        self.definitions
            .values()
            .filter(|d| d.release_priority >= min_priority)
            .collect()
    }

    /// Get definitions by provider
    pub fn by_provider(&self, provider: &str) -> Vec<&Definition> {
        self.definitions
            .values()
            .filter(|d| d.provider == provider)
            .collect()
    }

    /// Get all unique providers
    pub fn providers(&self) -> Vec<String> {
        let mut providers: Vec<String> = self.definitions
            .values()
            .map(|d| d.provider.clone())
            .collect();
        providers.sort();
        providers.dedup();
        providers
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

    #[test]
    fn test_label_parse() {
        let label = Label::parse("datetime.iso_8601").unwrap();
        assert_eq!(label.provider, "datetime");
        assert_eq!(label.method, "iso_8601");
        assert!(label.locale.is_none());
    }

    #[test]
    fn test_label_display() {
        let label = Label {
            provider: "internet".to_string(),
            method: "ip_v4".to_string(),
            locale: None,
        };
        assert_eq!(label.to_string(), "internet.ip_v4");
    }
}

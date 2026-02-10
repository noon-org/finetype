//! Inference utilities for text classification.

use crate::char_cnn::{CharCnn, CharCnnConfig, CharVocab};
use crate::model::{TextClassifier, TextClassifierConfig};
use candle_core::{DType, Device, Tensor};
use candle_nn::VarBuilder;
use finetype_core::{Taxonomy, Tokenizer};
use std::collections::HashMap;
use std::path::Path;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum InferenceError {
    #[error("Model error: {0}")]
    ModelError(#[from] candle_core::Error),
    #[error("Tokenizer error: {0}")]
    TokenizerError(#[from] finetype_core::tokenizer::TokenizerError),
    #[error("Taxonomy error: {0}")]
    TaxonomyError(#[from] finetype_core::taxonomy::TaxonomyError),
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("Invalid model path: {0}")]
    InvalidPath(String),
}

/// Classification result.
#[derive(Debug, Clone)]
pub struct ClassificationResult {
    pub label: String,
    pub confidence: f32,
    pub all_scores: Vec<(String, f32)>,
}

/// Classifier for text classification inference.
pub struct Classifier {
    model: TextClassifier,
    tokenizer: Tokenizer,
    index_to_label: HashMap<usize, String>,
    device: Device,
    max_seq_length: usize,
}

impl Classifier {
    /// Load a classifier from a directory containing model weights and taxonomy.
    pub fn load<P: AsRef<Path>>(model_dir: P) -> Result<Self, InferenceError> {
        let model_dir = model_dir.as_ref();

        // Determine device
        let device = Self::get_device();

        // Load taxonomy
        let taxonomy_path = model_dir.join("taxonomy.yaml");
        let taxonomy = if taxonomy_path.exists() {
            Taxonomy::from_file(&taxonomy_path)?
        } else {
            // Try default labels path
            Taxonomy::from_file(model_dir.join("labels.yaml"))?
        };

        let n_classes = taxonomy.len();
        let index_to_label = taxonomy.index_to_label();

        // Load config
        // TODO: Load from config file if available
        let config = TextClassifierConfig {
            n_classes,
            max_seq_length: 128, // Must match training config
            ..Default::default()
        };

        // Load model weights
        let weights_path = model_dir.join("model.safetensors");
        let vb = if weights_path.exists() {
            unsafe { VarBuilder::from_mmaped_safetensors(&[weights_path], DType::F32, &device)? }
        } else {
            // Initialize with random weights for testing
            VarBuilder::zeros(DType::F32, &device)
        };

        let model = TextClassifier::new(config.clone(), vb)?;
        let tokenizer = Tokenizer::bert_cased()?;

        Ok(Self {
            model,
            tokenizer,
            index_to_label,
            device,
            max_seq_length: config.max_seq_length,
        })
    }

    /// Classify a single text input.
    pub fn classify(&self, text: &str) -> Result<ClassificationResult, InferenceError> {
        let results = self.classify_batch(&[text.to_string()])?;
        Ok(results.into_iter().next().unwrap())
    }

    /// Classify multiple text inputs.
    pub fn classify_batch(
        &self,
        texts: &[String],
    ) -> Result<Vec<ClassificationResult>, InferenceError> {
        let batch_size = texts.len();

        // Tokenize all inputs
        let mut all_ids = Vec::with_capacity(batch_size);
        let mut all_masks = Vec::with_capacity(batch_size);

        for text in texts {
            let (ids, mask) = self.tokenizer.encode_padded(text, self.max_seq_length)?;
            all_ids.push(ids);
            all_masks.push(mask);
        }

        // Create tensors
        let input_ids = Tensor::new(
            all_ids.into_iter().flatten().collect::<Vec<u32>>(),
            &self.device,
        )?
        .reshape((batch_size, self.max_seq_length))?;

        let attention_mask = Tensor::new(
            all_masks.into_iter().flatten().collect::<Vec<u32>>(),
            &self.device,
        )?
        .reshape((batch_size, self.max_seq_length))?
        .to_dtype(DType::F32)?;

        // Run inference
        let probs = self.model.infer(&input_ids, Some(&attention_mask))?;
        let probs = probs.to_vec2::<f32>()?;

        // Convert to results
        let mut results = Vec::with_capacity(batch_size);
        for prob_row in probs {
            let (max_idx, max_prob) = prob_row
                .iter()
                .enumerate()
                .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
                .unwrap();

            let label = self
                .index_to_label
                .get(&max_idx)
                .cloned()
                .unwrap_or_else(|| "unknown".to_string());

            let all_scores: Vec<(String, f32)> = prob_row
                .iter()
                .enumerate()
                .map(|(i, &p)| {
                    let lbl = self
                        .index_to_label
                        .get(&i)
                        .cloned()
                        .unwrap_or_else(|| format!("class_{}", i));
                    (lbl, p)
                })
                .collect();

            results.push(ClassificationResult {
                label,
                confidence: *max_prob,
                all_scores,
            });
        }

        Ok(results)
    }

    /// Get the best device available.
    fn get_device() -> Device {
        #[cfg(feature = "cuda")]
        {
            if let Ok(device) = Device::new_cuda(0) {
                return device;
            }
        }

        #[cfg(feature = "metal")]
        {
            if let Ok(device) = Device::new_metal(0) {
                return device;
            }
        }

        Device::Cpu
    }

    /// Get the tokenizer.
    pub fn tokenizer(&self) -> &Tokenizer {
        &self.tokenizer
    }

    /// Get the device being used.
    pub fn device(&self) -> &Device {
        &self.device
    }
}

/// CharCNN-based classifier for text classification inference.
pub struct CharClassifier {
    model: CharCnn,
    vocab: CharVocab,
    index_to_label: HashMap<usize, String>,
    device: Device,
    max_seq_length: usize,
}

impl CharClassifier {
    /// Load a CharCNN classifier from embedded byte slices.
    ///
    /// Used by the DuckDB extension where model files are compiled into the binary.
    pub fn from_bytes(
        weights: &[u8],
        labels_json: &[u8],
        config_yaml: &[u8],
    ) -> Result<Self, InferenceError> {
        let device = Self::get_device();

        // Parse labels
        let labels_str = std::str::from_utf8(labels_json).map_err(|e| {
            InferenceError::InvalidPath(format!("Invalid UTF-8 in labels.json: {}", e))
        })?;
        let labels: Vec<String> = serde_json::from_str(labels_str).map_err(|e| {
            InferenceError::InvalidPath(format!("Failed to parse labels.json: {}", e))
        })?;
        let n_classes = labels.len();
        let index_to_label: HashMap<usize, String> = labels.into_iter().enumerate().collect();

        // Parse config
        let config_str = std::str::from_utf8(config_yaml).unwrap_or("");
        let mut vocab_size = 97usize;
        let mut max_seq_length = 128usize;
        let mut embed_dim = 32usize;
        let mut num_filters = 64usize;
        let mut hidden_dim = 128usize;

        for line in config_str.lines() {
            if let Some((key, val)) = line.split_once(':') {
                let key = key.trim();
                let val = val.trim();
                match key {
                    "vocab_size" => vocab_size = val.parse().unwrap_or(97),
                    "max_seq_length" => max_seq_length = val.parse().unwrap_or(128),
                    "embed_dim" => embed_dim = val.parse().unwrap_or(32),
                    "num_filters" => num_filters = val.parse().unwrap_or(64),
                    "hidden_dim" => hidden_dim = val.parse().unwrap_or(128),
                    _ => {}
                }
            }
        }

        let vocab = CharVocab::new();
        let config = CharCnnConfig {
            vocab_size,
            max_seq_length,
            embed_dim,
            num_filters,
            kernel_sizes: vec![2, 3, 4, 5],
            hidden_dim,
            n_classes,
            dropout: 0.0,
        };

        let vb = VarBuilder::from_buffered_safetensors(weights.to_vec(), DType::F32, &device)?;
        let model = CharCnn::new(config, vb)?;

        Ok(Self {
            model,
            vocab,
            index_to_label,
            device,
            max_seq_length,
        })
    }

    /// Load a CharCNN classifier from a directory.
    pub fn load<P: AsRef<Path>>(model_dir: P) -> Result<Self, InferenceError> {
        let model_dir = model_dir.as_ref();
        let device = Self::get_device();

        // Load label mapping — try labels.json first (saved by trainer), then taxonomy.yaml
        let labels_json_path = model_dir.join("labels.json");
        let taxonomy_path = model_dir.join("taxonomy.yaml");
        let (n_classes, index_to_label) = if labels_json_path.exists() {
            let content = std::fs::read_to_string(&labels_json_path)?;
            let labels: Vec<String> = serde_json::from_str(&content).map_err(|e| {
                InferenceError::InvalidPath(format!("Failed to parse labels.json: {}", e))
            })?;
            let n = labels.len();
            let mapping: HashMap<usize, String> = labels.into_iter().enumerate().collect();
            (n, mapping)
        } else if taxonomy_path.exists() {
            let taxonomy = Taxonomy::from_file(&taxonomy_path)?;
            let n = taxonomy.len();
            (n, taxonomy.index_to_label())
        } else {
            let labels_yaml_path = model_dir.join("labels.yaml");
            let taxonomy = Taxonomy::from_file(&labels_yaml_path)?;
            let n = taxonomy.len();
            (n, taxonomy.index_to_label())
        };

        // Load config from config.yaml if available
        let config_path = model_dir.join("config.yaml");
        let (vocab_size, max_seq_length, embed_dim, num_filters, hidden_dim) =
            if config_path.exists() {
                let config_str = std::fs::read_to_string(&config_path)?;
                let mut vocab_size = 97usize;
                let mut max_seq_length = 128usize;
                let mut embed_dim = 32usize;
                let mut num_filters = 64usize;
                let mut hidden_dim = 128usize;

                for line in config_str.lines() {
                    if let Some((key, val)) = line.split_once(':') {
                        let key = key.trim();
                        let val = val.trim();
                        match key {
                            "vocab_size" => vocab_size = val.parse().unwrap_or(97),
                            "max_seq_length" => max_seq_length = val.parse().unwrap_or(128),
                            "embed_dim" => embed_dim = val.parse().unwrap_or(32),
                            "num_filters" => num_filters = val.parse().unwrap_or(64),
                            "hidden_dim" => hidden_dim = val.parse().unwrap_or(128),
                            _ => {}
                        }
                    }
                }
                (
                    vocab_size,
                    max_seq_length,
                    embed_dim,
                    num_filters,
                    hidden_dim,
                )
            } else {
                (97, 128, 32, 64, 128)
            };

        let vocab = CharVocab::new();

        let config = CharCnnConfig {
            vocab_size,
            max_seq_length,
            embed_dim,
            num_filters,
            kernel_sizes: vec![2, 3, 4, 5],
            hidden_dim,
            n_classes,
            dropout: 0.0, // No dropout during inference
        };

        // Load model weights
        let weights_path = model_dir.join("model.safetensors");
        let vb =
            unsafe { VarBuilder::from_mmaped_safetensors(&[weights_path], DType::F32, &device)? };

        let model = CharCnn::new(config, vb)?;

        Ok(Self {
            model,
            vocab,
            index_to_label,
            device,
            max_seq_length,
        })
    }

    /// Classify a single text input.
    pub fn classify(&self, text: &str) -> Result<ClassificationResult, InferenceError> {
        let results = self.classify_batch(&[text.to_string()])?;
        Ok(results.into_iter().next().unwrap())
    }

    /// Classify multiple text inputs.
    pub fn classify_batch(
        &self,
        texts: &[String],
    ) -> Result<Vec<ClassificationResult>, InferenceError> {
        let batch_size = texts.len();

        // Encode all inputs
        let mut all_ids = Vec::with_capacity(batch_size * self.max_seq_length);
        for text in texts {
            let ids = self.vocab.encode(text, self.max_seq_length);
            all_ids.extend(ids);
        }

        // Create tensor
        let input_ids =
            Tensor::new(all_ids, &self.device)?.reshape((batch_size, self.max_seq_length))?;

        // Run inference
        let probs = self.model.infer(&input_ids)?;
        let probs = probs.to_vec2::<f32>()?;

        // Convert to results
        let mut results = Vec::with_capacity(batch_size);
        for prob_row in probs {
            let (max_idx, max_prob) = prob_row
                .iter()
                .enumerate()
                .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
                .unwrap();

            let label = self
                .index_to_label
                .get(&max_idx)
                .cloned()
                .unwrap_or_else(|| "unknown".to_string());

            let all_scores: Vec<(String, f32)> = prob_row
                .iter()
                .enumerate()
                .map(|(i, &p)| {
                    let lbl = self
                        .index_to_label
                        .get(&i)
                        .cloned()
                        .unwrap_or_else(|| format!("class_{}", i));
                    (lbl, p)
                })
                .collect();

            results.push(ClassificationResult {
                label,
                confidence: *max_prob,
                all_scores,
            });
        }

        // Post-process: apply format-based corrections for known model confusions
        for (result, text) in results.iter_mut().zip(texts.iter()) {
            post_process(result, text);
        }

        Ok(results)
    }

    /// Get the best device available.
    fn get_device() -> Device {
        #[cfg(feature = "cuda")]
        {
            if let Ok(device) = Device::new_cuda(0) {
                return device;
            }
        }

        #[cfg(feature = "metal")]
        {
            if let Ok(device) = Device::new_metal(0) {
                return device;
            }
        }

        Device::Cpu
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// POST-PROCESSING RULES
// ═══════════════════════════════════════════════════════════════════════════════

/// Apply format-based corrections for known model confusion pairs.
///
/// These rules check the actual input text to resolve confusions where the model
/// struggles but the format provides a definitive signal. Each rule is a simple
/// character/pattern check with no ambiguity.
fn post_process(result: &mut ClassificationResult, text: &str) {
    // Rule 1: rfc_3339 vs iso_8601_offset
    //
    // The only difference is T (ISO 8601) vs space (RFC 3339) between date and time.
    // The model confuses these 100% of the time. A simple character check resolves it.
    //
    // iso_8601_offset: "2024-01-15T10:30:00+05:00" (T separator)
    // rfc_3339:        "2024-01-15 10:30:00+05:00" (space separator)
    if result.label == "datetime.timestamp.iso_8601_offset"
        || result.label == "datetime.timestamp.rfc_3339"
    {
        let trimmed = text.trim();
        // Look for the separator at position 10 (after YYYY-MM-DD)
        if trimmed.len() >= 11 {
            let sep = trimmed.as_bytes()[10];
            if sep == b'T' {
                result.label = "datetime.timestamp.iso_8601_offset".to_string();
            } else if sep == b' ' {
                result.label = "datetime.timestamp.rfc_3339".to_string();
            }
        }
    }

    // Rule 2: hash vs token_hex
    //
    // Cryptographic hashes have fixed lengths: 32 (MD5), 40 (SHA-1), 64 (SHA-256), 128 (SHA-512).
    // Hex tokens have variable non-standard lengths. The model confuses these (58x token_hex→hash).
    // A simple length check on the trimmed hex string resolves this definitively.
    if result.label == "technology.cryptographic.hash"
        || result.label == "technology.cryptographic.token_hex"
    {
        let trimmed = text.trim();
        let is_hex = !trimmed.is_empty()
            && trimmed
                .bytes()
                .all(|b| b.is_ascii_hexdigit() && !b.is_ascii_uppercase());
        if is_hex {
            let len = trimmed.len();
            if len == 32 || len == 40 || len == 64 || len == 128 {
                result.label = "technology.cryptographic.hash".to_string();
            } else {
                result.label = "technology.cryptographic.token_hex".to_string();
            }
        }
    }

    // Rule 3: emoji vs gender_symbol
    //
    // Gender symbols are a specific set: ♂ (U+2642), ♀ (U+2640), ⚧ (U+26A7), ⚪ (U+26AA).
    // The model confuses emojis as gender symbols (96x). A character identity check resolves this.
    if result.label == "identity.person.gender_symbol"
        || result.label == "representation.text.emoji"
    {
        let trimmed = text.trim();
        let is_gender_symbol = trimmed.chars().count() == 1
            && matches!(trimmed.chars().next(), Some('♂' | '♀' | '⚧' | '⚪'));
        if is_gender_symbol {
            result.label = "identity.person.gender_symbol".to_string();
        } else if !trimmed.is_empty() {
            result.label = "representation.text.emoji".to_string();
        }
    }
}

#[cfg(test)]
mod post_process_tests {
    use super::*;

    fn make_result(label: &str) -> ClassificationResult {
        ClassificationResult {
            label: label.to_string(),
            confidence: 0.9,
            all_scores: vec![],
        }
    }

    #[test]
    fn test_iso_8601_offset_with_t_separator() {
        let mut result = make_result("datetime.timestamp.rfc_3339");
        post_process(&mut result, "2024-01-15T10:30:00+05:00");
        assert_eq!(result.label, "datetime.timestamp.iso_8601_offset");
    }

    #[test]
    fn test_rfc_3339_with_space_separator() {
        let mut result = make_result("datetime.timestamp.iso_8601_offset");
        post_process(&mut result, "2024-01-15 10:30:00+05:00");
        assert_eq!(result.label, "datetime.timestamp.rfc_3339");
    }

    #[test]
    fn test_correct_iso_8601_offset_unchanged() {
        let mut result = make_result("datetime.timestamp.iso_8601_offset");
        post_process(&mut result, "2024-01-15T10:30:00+05:00");
        assert_eq!(result.label, "datetime.timestamp.iso_8601_offset");
    }

    #[test]
    fn test_correct_rfc_3339_unchanged() {
        let mut result = make_result("datetime.timestamp.rfc_3339");
        post_process(&mut result, "2024-01-15 10:30:00+05:00");
        assert_eq!(result.label, "datetime.timestamp.rfc_3339");
    }

    #[test]
    fn test_unrelated_label_unchanged() {
        let mut result = make_result("technology.internet.ip_v4");
        post_process(&mut result, "192.168.1.1");
        assert_eq!(result.label, "technology.internet.ip_v4");
    }

    #[test]
    fn test_short_text_no_crash() {
        let mut result = make_result("datetime.timestamp.rfc_3339");
        post_process(&mut result, "short");
        // No crash, label unchanged (too short to check)
        assert_eq!(result.label, "datetime.timestamp.rfc_3339");
    }

    // ── Rule 2: hash vs token_hex ────────────────────────────────────────────

    #[test]
    fn test_hash_md5_length_32() {
        let mut result = make_result("technology.cryptographic.token_hex");
        post_process(&mut result, "5d41402abc4b2a76b9719d911017c592");
        assert_eq!(result.label, "technology.cryptographic.hash");
    }

    #[test]
    fn test_hash_sha1_length_40() {
        let mut result = make_result("technology.cryptographic.token_hex");
        post_process(&mut result, "aaf4c61ddcc5e8a2dabede0f3b482cd9aea9434d");
        assert_eq!(result.label, "technology.cryptographic.hash");
    }

    #[test]
    fn test_hash_sha256_length_64() {
        let mut result = make_result("technology.cryptographic.token_hex");
        post_process(
            &mut result,
            "2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824",
        );
        assert_eq!(result.label, "technology.cryptographic.hash");
    }

    #[test]
    fn test_token_hex_non_standard_length() {
        let mut result = make_result("technology.cryptographic.hash");
        post_process(&mut result, "a417b553b18d13027c23e8016c3466b81e70832254");
        // 42 chars — not a standard hash length, should become token_hex
        assert_eq!(result.label, "technology.cryptographic.token_hex");
    }

    #[test]
    fn test_correct_hash_unchanged() {
        let mut result = make_result("technology.cryptographic.hash");
        post_process(&mut result, "5d41402abc4b2a76b9719d911017c592");
        assert_eq!(result.label, "technology.cryptographic.hash");
    }

    #[test]
    fn test_correct_token_hex_unchanged() {
        let mut result = make_result("technology.cryptographic.token_hex");
        post_process(&mut result, "deadbeefcafebabe00ff11");
        // 22 chars — non-standard, stays as token_hex
        assert_eq!(result.label, "technology.cryptographic.token_hex");
    }

    #[test]
    fn test_hash_with_uppercase_not_reclassified() {
        // Uppercase hex isn't lowercase-only, so rule doesn't fire
        let mut result = make_result("technology.cryptographic.hash");
        post_process(&mut result, "5D41402ABC4B2A76B9719D911017C592");
        // Label unchanged — uppercase hex doesn't match our hex check
        assert_eq!(result.label, "technology.cryptographic.hash");
    }

    // ── Rule 3: emoji vs gender_symbol ───────────────────────────────────────

    #[test]
    fn test_gender_symbol_male() {
        let mut result = make_result("representation.text.emoji");
        post_process(&mut result, "♂");
        assert_eq!(result.label, "identity.person.gender_symbol");
    }

    #[test]
    fn test_gender_symbol_female() {
        let mut result = make_result("representation.text.emoji");
        post_process(&mut result, "♀");
        assert_eq!(result.label, "identity.person.gender_symbol");
    }

    #[test]
    fn test_gender_symbol_transgender() {
        let mut result = make_result("representation.text.emoji");
        post_process(&mut result, "⚧");
        assert_eq!(result.label, "identity.person.gender_symbol");
    }

    #[test]
    fn test_emoji_not_gender_symbol() {
        let mut result = make_result("identity.person.gender_symbol");
        post_process(&mut result, "🎉");
        assert_eq!(result.label, "representation.text.emoji");
    }

    #[test]
    fn test_emoji_rocket_not_gender_symbol() {
        let mut result = make_result("identity.person.gender_symbol");
        post_process(&mut result, "🚀");
        assert_eq!(result.label, "representation.text.emoji");
    }

    #[test]
    fn test_correct_emoji_unchanged() {
        let mut result = make_result("representation.text.emoji");
        post_process(&mut result, "😀");
        assert_eq!(result.label, "representation.text.emoji");
    }

    #[test]
    fn test_correct_gender_symbol_unchanged() {
        let mut result = make_result("identity.person.gender_symbol");
        post_process(&mut result, "♂");
        assert_eq!(result.label, "identity.person.gender_symbol");
    }
}

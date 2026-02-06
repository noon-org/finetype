//! Inference utilities for text classification.

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
        let config = TextClassifierConfig {
            n_classes,
            ..Default::default()
        };
        
        // Load model weights
        let weights_path = model_dir.join("model.safetensors");
        let vb = if weights_path.exists() {
            unsafe {
                VarBuilder::from_mmaped_safetensors(&[weights_path], DType::F32, &device)?
            }
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
    pub fn classify_batch(&self, texts: &[String]) -> Result<Vec<ClassificationResult>, InferenceError> {
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

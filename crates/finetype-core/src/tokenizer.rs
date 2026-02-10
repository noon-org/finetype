//! Tokenizer wrapper for text classification.
//!
//! Wraps the HuggingFace tokenizers library for consistent tokenization.

use thiserror::Error;
use tokenizers::Tokenizer as HfTokenizer;

#[derive(Error, Debug)]
pub enum TokenizerError {
    #[error("Failed to load tokenizer: {0}")]
    LoadError(String),
    #[error("Failed to encode text: {0}")]
    EncodeError(String),
    #[error("Failed to decode tokens: {0}")]
    DecodeError(String),
}

/// Tokenizer for text classification.
pub struct Tokenizer {
    inner: HfTokenizer,
}

impl Tokenizer {
    /// Load the default BERT tokenizer from HuggingFace Hub.
    pub fn bert_cased() -> Result<Self, TokenizerError> {
        let tokenizer = HfTokenizer::from_pretrained("bert-base-cased", None)
            .map_err(|e| TokenizerError::LoadError(e.to_string()))?;
        Ok(Self { inner: tokenizer })
    }

    /// Load a tokenizer from a file.
    pub fn from_file(path: &str) -> Result<Self, TokenizerError> {
        let tokenizer =
            HfTokenizer::from_file(path).map_err(|e| TokenizerError::LoadError(e.to_string()))?;
        Ok(Self { inner: tokenizer })
    }

    /// Encode text into token IDs.
    pub fn encode(&self, text: &str) -> Result<Vec<u32>, TokenizerError> {
        let encoding = self
            .inner
            .encode(text, true)
            .map_err(|e| TokenizerError::EncodeError(e.to_string()))?;
        Ok(encoding.get_ids().to_vec())
    }

    /// Encode text with attention mask.
    pub fn encode_with_mask(&self, text: &str) -> Result<(Vec<u32>, Vec<u32>), TokenizerError> {
        let encoding = self
            .inner
            .encode(text, true)
            .map_err(|e| TokenizerError::EncodeError(e.to_string()))?;
        Ok((
            encoding.get_ids().to_vec(),
            encoding.get_attention_mask().to_vec(),
        ))
    }

    /// Encode and pad to a fixed length.
    pub fn encode_padded(
        &self,
        text: &str,
        max_length: usize,
    ) -> Result<(Vec<u32>, Vec<u32>), TokenizerError> {
        let (mut ids, mut mask) = self.encode_with_mask(text)?;

        // Truncate if too long
        if ids.len() > max_length {
            ids.truncate(max_length);
            mask.truncate(max_length);
        }

        // Pad if too short
        while ids.len() < max_length {
            ids.push(self.pad_token_id());
            mask.push(0);
        }

        Ok((ids, mask))
    }

    /// Decode token IDs back to text.
    pub fn decode(&self, ids: &[u32]) -> Result<String, TokenizerError> {
        self.inner
            .decode(ids, true)
            .map_err(|e| TokenizerError::DecodeError(e.to_string()))
    }

    /// Get the vocabulary size.
    pub fn vocab_size(&self) -> usize {
        self.inner.get_vocab_size(true)
    }

    /// Get the padding token ID.
    pub fn pad_token_id(&self) -> u32 {
        self.inner.token_to_id("[PAD]").unwrap_or(0)
    }

    /// Get the CLS token ID.
    pub fn cls_token_id(&self) -> u32 {
        self.inner.token_to_id("[CLS]").unwrap_or(101)
    }

    /// Get the SEP token ID.
    pub fn sep_token_id(&self) -> u32 {
        self.inner.token_to_id("[SEP]").unwrap_or(102)
    }
}

impl Default for Tokenizer {
    fn default() -> Self {
        Self::bert_cased().expect("Failed to load default tokenizer")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode_decode() {
        let tokenizer = Tokenizer::bert_cased().unwrap();
        let text = "Hello, world!";
        let ids = tokenizer.encode(text).unwrap();
        assert!(!ids.is_empty());

        let decoded = tokenizer.decode(&ids).unwrap();
        assert!(decoded.contains("Hello"));
    }

    #[test]
    fn test_padded_encoding() {
        let tokenizer = Tokenizer::bert_cased().unwrap();
        let (ids, mask) = tokenizer.encode_padded("test", 10).unwrap();
        assert_eq!(ids.len(), 10);
        assert_eq!(mask.len(), 10);
    }
}

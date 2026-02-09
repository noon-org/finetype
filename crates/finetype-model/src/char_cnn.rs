//! Character-level CNN for text classification.
//!
//! Architecture:
//! - Character embedding
//! - Multiple parallel 1D convolutions (kernel sizes 2,3,4,5)
//! - Max pooling over sequence
//! - Fully connected layers
//! - Softmax classifier

use candle_core::{DType, Device, Module, Result, Tensor};
use candle_nn::{conv1d, embedding, linear, Conv1d, Conv1dConfig, Embedding, Linear, VarBuilder};

/// Character vocabulary for the model.
pub struct CharVocab {
    char_to_idx: std::collections::HashMap<char, u32>,
    vocab_size: usize,
}

impl CharVocab {
    /// Create a new character vocabulary.
    pub fn new() -> Self {
        let mut char_to_idx = std::collections::HashMap::new();
        let mut idx = 1u32; // 0 reserved for padding
        
        // Lowercase letters
        for c in 'a'..='z' {
            char_to_idx.insert(c, idx);
            idx += 1;
        }
        // Uppercase letters
        for c in 'A'..='Z' {
            char_to_idx.insert(c, idx);
            idx += 1;
        }
        // Digits
        for c in '0'..='9' {
            char_to_idx.insert(c, idx);
            idx += 1;
        }
        // Common punctuation and special characters
        for c in " .-_@:/\\#$%&*+='\"<>()[]{}|~`!?,;^".chars() {
            char_to_idx.insert(c, idx);
            idx += 1;
        }
        
        let vocab_size = idx as usize + 1; // +1 for unknown
        
        Self { char_to_idx, vocab_size }
    }
    
    /// Encode a string to character indices.
    pub fn encode(&self, text: &str, max_len: usize) -> Vec<u32> {
        let mut ids = Vec::with_capacity(max_len);
        for c in text.chars().take(max_len) {
            let idx = self.char_to_idx.get(&c).copied().unwrap_or(self.vocab_size as u32 - 1);
            ids.push(idx);
        }
        // Pad to max_len
        while ids.len() < max_len {
            ids.push(0); // padding
        }
        ids
    }
    
    /// Get vocabulary size.
    pub fn vocab_size(&self) -> usize {
        self.vocab_size
    }
}

impl Default for CharVocab {
    fn default() -> Self {
        Self::new()
    }
}

/// Configuration for the character-level CNN.
#[derive(Debug, Clone)]
pub struct CharCnnConfig {
    pub vocab_size: usize,
    pub max_seq_length: usize,
    pub embed_dim: usize,
    pub num_filters: usize,
    pub kernel_sizes: Vec<usize>,
    pub hidden_dim: usize,
    pub n_classes: usize,
    pub dropout: f64,
}

impl Default for CharCnnConfig {
    fn default() -> Self {
        Self {
            vocab_size: 100,
            max_seq_length: 128,
            embed_dim: 32,
            num_filters: 64,
            kernel_sizes: vec![2, 3, 4, 5],
            hidden_dim: 128,
            n_classes: 100,
            dropout: 0.3,
        }
    }
}

/// Character-level CNN classifier.
pub struct CharCnn {
    char_embedding: Embedding,
    convs: Vec<Conv1d>,
    fc1: Linear,
    fc2: Linear,
    config: CharCnnConfig,
}

impl CharCnn {
    /// Create a new character-level CNN.
    pub fn new(config: CharCnnConfig, vb: VarBuilder) -> Result<Self> {
        let char_embedding = embedding(
            config.vocab_size,
            config.embed_dim,
            vb.pp("char_emb"),
        )?;
        
        // Create convolutions with different kernel sizes
        let mut convs = Vec::with_capacity(config.kernel_sizes.len());
        for (i, &kernel_size) in config.kernel_sizes.iter().enumerate() {
            let conv_config = Conv1dConfig {
                padding: 0,
                stride: 1,
                dilation: 1,
                groups: 1,
            };
            let conv = conv1d(
                config.embed_dim,
                config.num_filters,
                kernel_size,
                conv_config,
                vb.pp(format!("conv_{}", i)),
            )?;
            convs.push(conv);
        }
        
        // Total features = num_filters * num_kernel_sizes
        let total_filters = config.num_filters * config.kernel_sizes.len();
        
        let fc1 = linear(total_filters, config.hidden_dim, vb.pp("fc1"))?;
        let fc2 = linear(config.hidden_dim, config.n_classes, vb.pp("fc2"))?;
        
        Ok(Self {
            char_embedding,
            convs,
            fc1,
            fc2,
            config,
        })
    }
    
    /// Forward pass.
    pub fn forward(&self, input_ids: &Tensor) -> Result<Tensor> {
        let (batch_size, _seq_len) = input_ids.dims2()?;
        
        // Character embeddings: (batch, seq_len) -> (batch, seq_len, embed_dim)
        let emb = self.char_embedding.forward(input_ids)?;
        
        // Conv1d expects (batch, channels, seq_len), so transpose
        let emb = emb.transpose(1, 2)?; // (batch, embed_dim, seq_len)
        
        // Apply each convolution and max pool
        let mut pooled_outputs = Vec::with_capacity(self.convs.len());
        for conv in &self.convs {
            // Conv: (batch, embed_dim, seq_len) -> (batch, num_filters, seq_len - kernel + 1)
            let conv_out = conv.forward(&emb)?;
            let conv_out = conv_out.relu()?;
            
            // Global max pool over sequence dimension
            let pooled = conv_out.max(2)?; // (batch, num_filters)
            pooled_outputs.push(pooled);
        }
        
        // Concatenate all pooled outputs
        let concat = Tensor::cat(&pooled_outputs, 1)?; // (batch, num_filters * num_kernels)
        
        // Fully connected layers
        let hidden = self.fc1.forward(&concat)?;
        let hidden = hidden.relu()?;
        let logits = self.fc2.forward(&hidden)?;
        
        Ok(logits)
    }
    
    /// Inference with softmax probabilities.
    pub fn infer(&self, input_ids: &Tensor) -> Result<Tensor> {
        let logits = self.forward(input_ids)?;
        candle_nn::ops::softmax(&logits, 1)
    }
    
    /// Get config.
    pub fn config(&self) -> &CharCnnConfig {
        &self.config
    }
}

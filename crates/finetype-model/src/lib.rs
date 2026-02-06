//! FineType Model
//!
//! Candle-based transformer model for semantic type classification.

pub mod model;
pub mod inference;
pub mod training;

pub use model::{TextClassifier, TextClassifierConfig};
pub use inference::Classifier;
pub use training::{Trainer, TrainingConfig, TrainingError};

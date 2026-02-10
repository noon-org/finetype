//! FineType Model
//!
//! Candle-based models for semantic type classification.

pub mod char_cnn;
pub mod char_training;
pub mod inference;
pub mod model;
pub mod training;

pub use char_cnn::{CharCnn, CharCnnConfig, CharVocab};
pub use char_training::{CharTrainer, CharTrainingConfig};
pub use inference::{CharClassifier, ClassificationResult, Classifier};
pub use model::{TextClassifier, TextClassifierConfig};
pub use training::{Trainer, TrainingConfig, TrainingError};

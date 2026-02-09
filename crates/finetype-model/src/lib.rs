//! FineType Model
//!
//! Candle-based models for semantic type classification.

pub mod model;
pub mod char_cnn;
pub mod inference;
pub mod training;
pub mod char_training;

pub use model::{TextClassifier, TextClassifierConfig};
pub use char_cnn::{CharCnn, CharCnnConfig, CharVocab};
pub use inference::{Classifier, CharClassifier, ClassificationResult};
pub use training::{Trainer, TrainingConfig, TrainingError};
pub use char_training::{CharTrainer, CharTrainingConfig};

//! FineType Core
//!
//! Core library for precision format detection taxonomy and data generation.
//!
//! - `taxonomy` — domain.category.type label format with transformation contracts
//! - `generator` — synthetic data generation for all 151 types
//! - `tokenizer` — text tokenization for model training

pub mod taxonomy;
pub mod generator;
pub mod tokenizer;

pub use taxonomy::{Definition, Designation, Label, Taxonomy};
pub use generator::{Generator, Sample};
pub use tokenizer::Tokenizer;

//! FineType Core
//!
//! Core library for semantic type classification taxonomy and data generation.

pub mod taxonomy;
pub mod generator;
pub mod tokenizer;

pub use taxonomy::{Definition, Designation, Label, Locale, Taxonomy};
pub use generator::Generator;
pub use tokenizer::Tokenizer;

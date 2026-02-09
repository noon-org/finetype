//! FineType CLI
//!
//! Command-line interface for precision format detection.

use anyhow::Result;
use clap::{Parser, Subcommand};
use finetype_core::{Generator, Taxonomy};
use finetype_model::Classifier;
use serde_json::json;
use std::io::{self, BufRead, Write};
use std::path::PathBuf;
use tracing_subscriber::EnvFilter;

#[derive(Parser)]
#[command(name = "finetype")]
#[command(author = "Hugh Cameron")]
#[command(version)]
#[command(about = "Precision format detection for text data", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Classify text input
    Infer {
        /// Single text input
        #[arg(short, long)]
        input: Option<String>,

        /// File containing inputs (one per line)
        #[arg(short, long)]
        file: Option<PathBuf>,

        /// Model directory
        #[arg(short, long, default_value = "models/default")]
        model: PathBuf,

        /// Output format (plain, json, csv)
        #[arg(short, long, default_value = "plain")]
        output: OutputFormat,

        /// Include confidence score
        #[arg(long)]
        confidence: bool,

        /// Include input value in output
        #[arg(short, long)]
        value: bool,

        /// Model type (transformer, char_cnn)
        #[arg(long, default_value = "char-cnn")]
        model_type: ModelType,
    },

    /// Generate synthetic training data
    Generate {
        /// Number of samples per label
        #[arg(short, long, default_value = "100")]
        samples: usize,

        /// Minimum release priority
        #[arg(short, long, default_value = "3")]
        priority: u8,

        /// Output file
        #[arg(short, long, default_value = "training.ndjson")]
        output: PathBuf,

        /// Taxonomy file or directory
        #[arg(short, long, default_value = "labels")]
        taxonomy: PathBuf,

        /// Random seed for reproducibility
        #[arg(long, default_value = "42")]
        seed: u64,
    },

    /// Train a model
    Train {
        /// Training data file (NDJSON)
        #[arg(short, long)]
        data: PathBuf,

        /// Taxonomy file or directory
        #[arg(short, long, default_value = "labels")]
        taxonomy: PathBuf,

        /// Output directory for model
        #[arg(short, long, default_value = "models/default")]
        output: PathBuf,

        /// Number of epochs
        #[arg(short, long, default_value = "5")]
        epochs: usize,

        /// Batch size
        #[arg(short, long, default_value = "32")]
        batch_size: usize,

        /// Device (cpu, cuda, metal)
        #[arg(long, default_value = "cpu")]
        device: String,

        /// Model type (transformer, char_cnn)
        #[arg(long, default_value = "char-cnn")]
        model_type: ModelType,
    },

    /// Show taxonomy information
    Taxonomy {
        /// Taxonomy file or directory
        #[arg(short, long, default_value = "labels")]
        file: PathBuf,

        /// Filter by domain
        #[arg(short, long)]
        domain: Option<String>,

        /// Filter by category
        #[arg(short, long)]
        category: Option<String>,

        /// Minimum release priority
        #[arg(long)]
        priority: Option<u8>,

        /// Output format (plain, json, csv)
        #[arg(short, long, default_value = "plain")]
        output: OutputFormat,
    },
}

#[derive(Clone, Copy, Debug, clap::ValueEnum)]
enum OutputFormat {
    Plain,
    Json,
    Csv,
}

#[derive(Clone, Copy, Debug, clap::ValueEnum)]
enum ModelType {
    Transformer,
    CharCnn,
}

fn main() -> Result<()> {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let cli = Cli::parse();

    match cli.command {
        Commands::Infer {
            input,
            file,
            model,
            output,
            confidence,
            value,
            model_type,
        } => cmd_infer(input, file, model, output, confidence, value, model_type),

        Commands::Generate {
            samples,
            priority,
            output,
            taxonomy,
            seed,
        } => cmd_generate(samples, priority, output, taxonomy, seed),

        Commands::Train {
            data,
            taxonomy,
            output,
            epochs,
            batch_size,
            device,
            model_type,
        } => cmd_train(data, taxonomy, output, epochs, batch_size, device, model_type),

        Commands::Taxonomy {
            file,
            domain,
            category,
            priority,
            output,
        } => cmd_taxonomy(file, domain, category, priority, output),
    }
}

fn cmd_infer(
    input: Option<String>,
    file: Option<PathBuf>,
    model: PathBuf,
    output: OutputFormat,
    show_confidence: bool,
    show_value: bool,
    model_type: ModelType,
) -> Result<()> {
    use finetype_model::{CharClassifier, ClassificationResult};

    // Collect inputs
    let inputs: Vec<String> = if let Some(text) = input {
        vec![text]
    } else if let Some(path) = file {
        std::fs::read_to_string(path)?
            .lines()
            .map(String::from)
            .filter(|s| !s.is_empty())
            .collect()
    } else {
        // Read from stdin
        io::stdin()
            .lock()
            .lines()
            .filter_map(|l| l.ok())
            .filter(|s| !s.is_empty())
            .collect()
    };

    if inputs.is_empty() {
        eprintln!("No input provided");
        return Ok(());
    }

    // Helper to output result
    fn output_result(
        text: &str,
        result: &ClassificationResult,
        output: OutputFormat,
        show_value: bool,
        show_confidence: bool,
    ) {
        match output {
            OutputFormat::Plain => {
                if show_value && show_confidence {
                    println!("{}\t{}\t{:.4}", text, result.label, result.confidence);
                } else if show_value {
                    println!("{}\t{}", text, result.label);
                } else if show_confidence {
                    println!("{}\t{:.4}", result.label, result.confidence);
                } else {
                    println!("{}", result.label);
                }
            }
            OutputFormat::Json => {
                let mut obj = serde_json::Map::new();
                obj.insert("class".to_string(), json!(result.label));
                if show_value {
                    obj.insert("input".to_string(), json!(text));
                }
                if show_confidence {
                    obj.insert("confidence".to_string(), json!(result.confidence));
                }
                println!("{}", serde_json::Value::Object(obj));
            }
            OutputFormat::Csv => {
                if show_value && show_confidence {
                    println!("\"{}\",\"{}\",{:.4}", text, result.label, result.confidence);
                } else if show_value {
                    println!("\"{}\",\"{}\"", text, result.label);
                } else if show_confidence {
                    println!("\"{}\",{:.4}", result.label, result.confidence);
                } else {
                    println!("\"{}\"", result.label);
                }
            }
        }
    }

    match model_type {
        ModelType::Transformer => {
            let classifier = Classifier::load(&model)?;
            for text in inputs {
                let result = classifier.classify(&text)?;
                output_result(&text, &result, output, show_value, show_confidence);
            }
        }
        ModelType::CharCnn => {
            let classifier = CharClassifier::load(&model)?;
            for text in inputs {
                let result = classifier.classify(&text)?;
                output_result(&text, &result, output, show_value, show_confidence);
            }
        }
    }

    Ok(())
}

// ═══════════════════════════════════════════════════════════════════════════════
// GENERATE — Create synthetic training data
// ═══════════════════════════════════════════════════════════════════════════════

fn cmd_generate(
    samples: usize,
    priority: u8,
    output: PathBuf,
    taxonomy_path: PathBuf,
    seed: u64,
) -> Result<()> {
    eprintln!("Loading taxonomy from {:?}", taxonomy_path);

    let taxonomy = load_taxonomy(&taxonomy_path)?;

    eprintln!(
        "Loaded {} label definitions across {} domains",
        taxonomy.len(),
        taxonomy.domains().len()
    );

    eprintln!(
        "Generating {} samples per label (priority >= {})",
        samples, priority
    );

    let mut generator = Generator::with_seed(taxonomy, seed);
    let all_samples = generator.generate_all(priority, samples);

    eprintln!("Generated {} total samples", all_samples.len());

    // Write to file
    let mut file = std::fs::File::create(&output)?;
    for sample in all_samples {
        let record = json!({
            "text": sample.text,
            "classification": sample.label,
        });
        writeln!(file, "{}", record)?;
    }

    eprintln!("Saved to {:?}", output);
    Ok(())
}

// ═══════════════════════════════════════════════════════════════════════════════
// TRAIN — Train a classification model
// ═══════════════════════════════════════════════════════════════════════════════

fn cmd_train(
    data: PathBuf,
    taxonomy_path: PathBuf,
    output: PathBuf,
    epochs: usize,
    batch_size: usize,
    _device: String,
    model_type: ModelType,
) -> Result<()> {
    use finetype_core::Sample;
    use std::io::BufRead;

    eprintln!("Loading taxonomy from {:?}", taxonomy_path);
    let taxonomy = load_taxonomy(&taxonomy_path)?;
    eprintln!("Loaded {} label definitions", taxonomy.len());

    eprintln!("Loading training data from {:?}", data);
    let file = std::fs::File::open(&data)?;
    let reader = std::io::BufReader::new(file);

    let mut samples = Vec::new();
    for line in reader.lines() {
        let line = line?;
        if line.is_empty() {
            continue;
        }
        let record: serde_json::Value = serde_json::from_str(&line)?;
        let text = record["text"].as_str().unwrap_or("").to_string();
        let label = record["classification"].as_str().unwrap_or("").to_string();
        samples.push(Sample { text, label });
    }
    eprintln!("Loaded {} training samples", samples.len());

    match model_type {
        ModelType::Transformer => {
            use finetype_model::{Trainer, TrainingConfig};

            let config = TrainingConfig {
                batch_size,
                epochs,
                learning_rate: 1e-4,
                max_seq_length: 128,
                warmup_steps: 100,
                weight_decay: 0.01,
            };

            eprintln!("Training Transformer model");
            eprintln!("Training config: {:?}", config);

            let trainer = Trainer::new(config);
            trainer.train(&taxonomy, &samples, &output)?;
        }
        ModelType::CharCnn => {
            use finetype_model::{CharTrainer, CharTrainingConfig};

            let config = CharTrainingConfig {
                batch_size,
                epochs,
                learning_rate: 1e-3,
                max_seq_length: 128,
                embed_dim: 32,
                num_filters: 64,
                hidden_dim: 128,
                weight_decay: 1e-4,
                shuffle: true,
            };

            eprintln!("Training CharCNN model");
            eprintln!("Training config: {:?}", config);

            let trainer = CharTrainer::new(config);
            trainer.train(&taxonomy, &samples, &output)?;
        }
    }

    eprintln!("Training complete! Model saved to {:?}", output);
    Ok(())
}

// ═══════════════════════════════════════════════════════════════════════════════
// TAXONOMY — Display taxonomy information
// ═══════════════════════════════════════════════════════════════════════════════

fn cmd_taxonomy(
    file: PathBuf,
    domain: Option<String>,
    category: Option<String>,
    priority: Option<u8>,
    output: OutputFormat,
) -> Result<()> {
    let taxonomy = load_taxonomy(&file)?;

    // Collect matching definitions
    let mut defs: Vec<(&String, &finetype_core::Definition)> =
        if let (Some(dom), Some(cat)) = (&domain, &category) {
            taxonomy.by_category(dom, cat)
        } else if let Some(dom) = &domain {
            taxonomy.by_domain(dom)
        } else if let Some(prio) = priority {
            taxonomy.at_priority(prio)
        } else {
            taxonomy.definitions().collect()
        };

    // Apply priority filter even when domain/category is set
    if let Some(prio) = priority {
        defs.retain(|(_, d)| d.release_priority >= prio);
    }

    defs.sort_by_key(|(k, _)| k.clone());

    match output {
        OutputFormat::Plain => {
            println!("Domains: {:?}", taxonomy.domains());
            println!("Total labels: {}", taxonomy.len());
            if let Some(dom) = &domain {
                println!("Categories in {}: {:?}", dom, taxonomy.categories(dom));
            }
            println!();

            for (key, def) in &defs {
                let broad = def.broad_type.as_deref().unwrap_or("?");
                println!(
                    "{} \u{2192} {} (priority: {}, {:?})",
                    key, broad, def.release_priority, def.designation
                );
                if let Some(title) = &def.title {
                    println!("  {}", title);
                }
            }

            println!("\n{} definitions shown", defs.len());
        }
        OutputFormat::Json => {
            let labels: Vec<_> = defs
                .iter()
                .map(|(key, d)| {
                    json!({
                        "key": key,
                        "title": d.title,
                        "broad_type": d.broad_type,
                        "designation": format!("{:?}", d.designation),
                        "priority": d.release_priority,
                        "transform": d.transform,
                        "locales": d.locales,
                    })
                })
                .collect();
            println!("{}", serde_json::to_string_pretty(&labels)?);
        }
        OutputFormat::Csv => {
            println!("key,broad_type,priority,designation,title");
            for (key, def) in &defs {
                println!(
                    "\"{}\",\"{}\",{},\"{:?}\",\"{}\"",
                    key,
                    def.broad_type.as_deref().unwrap_or(""),
                    def.release_priority,
                    def.designation,
                    def.title.as_deref().unwrap_or("")
                );
            }
        }
    }

    Ok(())
}

// ═══════════════════════════════════════════════════════════════════════════════
// HELPERS
// ═══════════════════════════════════════════════════════════════════════════════

/// Load taxonomy from a file or directory.
fn load_taxonomy(path: &PathBuf) -> Result<Taxonomy> {
    if path.is_dir() {
        Ok(Taxonomy::from_directory(path)?)
    } else {
        Ok(Taxonomy::from_file(path)?)
    }
}

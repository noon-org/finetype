//! FineType CLI
//!
//! Command-line interface for semantic type classification.

use anyhow::Result;
use clap::{Parser, Subcommand};
use finetype_core::{Generator, Taxonomy};
use finetype_model::{Classifier};
use serde_json::json;
use std::io::{self, BufRead, Write};
use std::path::PathBuf;
use tracing_subscriber::EnvFilter;

#[derive(Parser)]
#[command(name = "finetype")]
#[command(author = "Hugh Cameron")]
#[command(version)]
#[command(about = "Semantic type classification for text data", long_about = None)]
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

        /// Taxonomy file
        #[arg(short, long, default_value = "labels/definitions.yaml")]
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

        /// Taxonomy file
        #[arg(short, long, default_value = "labels/definitions.yaml")]
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
    },

    /// Show taxonomy information
    Taxonomy {
        /// Taxonomy file
        #[arg(short, long, default_value = "labels/definitions.yaml")]
        file: PathBuf,

        /// Filter by provider
        #[arg(short, long)]
        provider: Option<String>,

        /// Minimum release priority
        #[arg(long)]
        priority: Option<u8>,

        /// Output format (plain, json)
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
        } => cmd_infer(input, file, model, output, confidence, value),

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
        } => cmd_train(data, taxonomy, output, epochs, batch_size, device),

        Commands::Taxonomy {
            file,
            provider,
            priority,
            output,
        } => cmd_taxonomy(file, provider, priority, output),
    }
}

fn cmd_infer(
    input: Option<String>,
    file: Option<PathBuf>,
    model: PathBuf,
    output: OutputFormat,
    show_confidence: bool,
    show_value: bool,
) -> Result<()> {
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

    // Load classifier
    let classifier = Classifier::load(&model)?;

    // Classify
    for text in inputs {
        let result = classifier.classify(&text)?;

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

    Ok(())
}

fn cmd_generate(
    samples: usize,
    priority: u8,
    output: PathBuf,
    taxonomy_path: PathBuf,
    seed: u64,
) -> Result<()> {
    eprintln!("Loading taxonomy from {:?}", taxonomy_path);
    let taxonomy = Taxonomy::from_file(&taxonomy_path)?;

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

fn cmd_train(
    data: PathBuf,
    taxonomy_path: PathBuf,
    output: PathBuf,
    epochs: usize,
    batch_size: usize,
    device: String,
) -> Result<()> {
    eprintln!("Training not yet fully implemented");
    eprintln!("Data: {:?}", data);
    eprintln!("Taxonomy: {:?}", taxonomy_path);
    eprintln!("Output: {:?}", output);
    eprintln!("Epochs: {}", epochs);
    eprintln!("Batch size: {}", batch_size);
    eprintln!("Device: {}", device);

    // TODO: Implement full training loop
    // For now, this is a placeholder

    Ok(())
}

fn cmd_taxonomy(
    file: PathBuf,
    provider: Option<String>,
    priority: Option<u8>,
    output: OutputFormat,
) -> Result<()> {
    let taxonomy = Taxonomy::from_file(&file)?;

    let definitions: Vec<_> = if let Some(prov) = &provider {
        taxonomy.by_provider(prov)
    } else if let Some(prio) = priority {
        taxonomy.at_priority(prio)
    } else {
        taxonomy.definitions().map(|(_, d)| d).collect()
    };

    match output {
        OutputFormat::Plain => {
            println!("Providers: {:?}", taxonomy.providers());
            println!("Total labels: {}", taxonomy.len());
            println!();

            for def in definitions {
                println!(
                    "{}.{} (priority: {}, designation: {:?})",
                    def.provider, def.method, def.release_priority, def.designation
                );
                if let Some(title) = &def.title {
                    println!("  {}", title);
                }
            }
        }
        OutputFormat::Json => {
            let labels: Vec<_> = definitions
                .iter()
                .map(|d| {
                    json!({
                        "label": d.label(),
                        "provider": d.provider,
                        "method": d.method,
                        "priority": d.release_priority,
                        "designation": format!("{:?}", d.designation),
                        "title": d.title,
                    })
                })
                .collect();
            println!("{}", serde_json::to_string_pretty(&labels)?);
        }
        OutputFormat::Csv => {
            println!("label,provider,method,priority,designation,title");
            for def in definitions {
                println!(
                    "\"{}\",\"{}\",\"{}\",{},\"{:?}\",\"{}\"",
                    def.label(),
                    def.provider,
                    def.method,
                    def.release_priority,
                    def.designation,
                    def.title.as_deref().unwrap_or("")
                );
            }
        }
    }

    Ok(())
}

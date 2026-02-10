//! FineType CLI
//!
//! Command-line interface for precision format detection.

use anyhow::Result;
use clap::{Parser, Subcommand};
use finetype_core::{format_report, Checker, Generator, Taxonomy};
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

        /// Inference mode: row (per-value) or column (distribution-based disambiguation)
        #[arg(long, default_value = "row")]
        mode: InferenceMode,

        /// Sample size for column mode (default 100)
        #[arg(long, default_value = "100")]
        sample_size: usize,
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

    /// Validate generator ↔ taxonomy alignment
    Check {
        /// Taxonomy file or directory
        #[arg(short, long, default_value = "labels")]
        taxonomy: PathBuf,

        /// Number of samples to generate per definition
        #[arg(short, long, default_value = "50")]
        samples: usize,

        /// Random seed for reproducibility
        #[arg(long, default_value = "42")]
        seed: u64,

        /// Minimum release priority to check (0 = all)
        #[arg(short, long)]
        priority: Option<u8>,

        /// Show verbose failure details
        #[arg(short, long)]
        verbose: bool,

        /// Output format (plain, json)
        #[arg(short, long, default_value = "plain")]
        output: OutputFormat,
    },

    /// Evaluate model accuracy on a test set
    Eval {
        /// Test data file (NDJSON with "text" and "classification" fields)
        #[arg(short, long)]
        data: PathBuf,

        /// Model directory
        #[arg(short, long, default_value = "models/default")]
        model: PathBuf,

        /// Taxonomy file or directory
        #[arg(short, long, default_value = "labels")]
        taxonomy: PathBuf,

        /// Model type (transformer, char_cnn)
        #[arg(long, default_value = "char-cnn")]
        model_type: ModelType,

        /// Number of top confusions to show
        #[arg(long, default_value = "20")]
        top_confusions: usize,

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

#[derive(Clone, Copy, Debug, clap::ValueEnum)]
enum ModelType {
    Transformer,
    CharCnn,
    Tiered,
}

#[derive(Clone, Copy, Debug, clap::ValueEnum)]
enum InferenceMode {
    /// Classify each value independently (default)
    Row,
    /// Treat all inputs as one column, use distribution to disambiguate
    Column,
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
            mode,
            sample_size,
        } => cmd_infer(
            input,
            file,
            model,
            output,
            confidence,
            value,
            model_type,
            mode,
            sample_size,
        ),

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
        } => cmd_train(
            data, taxonomy, output, epochs, batch_size, device, model_type,
        ),

        Commands::Taxonomy {
            file,
            domain,
            category,
            priority,
            output,
        } => cmd_taxonomy(file, domain, category, priority, output),

        Commands::Check {
            taxonomy,
            samples,
            seed,
            priority,
            verbose,
            output,
        } => cmd_check(taxonomy, samples, seed, priority, verbose, output),

        Commands::Eval {
            data,
            model,
            taxonomy,
            model_type,
            top_confusions,
            output,
        } => cmd_eval(data, model, taxonomy, model_type, top_confusions, output),
    }
}

#[allow(clippy::too_many_arguments)]
fn cmd_infer(
    input: Option<String>,
    file: Option<PathBuf>,
    model: PathBuf,
    output: OutputFormat,
    show_confidence: bool,
    show_value: bool,
    model_type: ModelType,
    mode: InferenceMode,
    sample_size: usize,
) -> Result<()> {
    use finetype_model::{CharClassifier, ClassificationResult, ColumnClassifier, ColumnConfig};

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
            .map_while(|l| l.ok())
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

    // Column mode: treat all inputs as one column, return single prediction
    if matches!(mode, InferenceMode::Column) {
        match model_type {
            ModelType::CharCnn => {
                let classifier = CharClassifier::load(&model)?;
                let config = ColumnConfig {
                    sample_size,
                    ..Default::default()
                };
                let column_classifier = ColumnClassifier::new(classifier, config);
                let result = column_classifier.classify_column(&inputs)?;

                match output {
                    OutputFormat::Plain => {
                        println!("{}", result.label);
                        if show_confidence {
                            println!(
                                "  confidence: {:.4} ({} samples)",
                                result.confidence, result.samples_used
                            );
                        }
                        if result.disambiguation_applied {
                            println!(
                                "  disambiguation: {}",
                                result.disambiguation_rule.as_deref().unwrap_or("unknown")
                            );
                        }
                        if show_value {
                            println!("  vote distribution:");
                            for (label, frac) in &result.vote_distribution {
                                if *frac >= 0.01 {
                                    println!("    {:.1}%  {}", frac * 100.0, label);
                                }
                            }
                        }
                    }
                    OutputFormat::Json => {
                        let mut obj = serde_json::Map::new();
                        obj.insert("class".to_string(), json!(result.label));
                        obj.insert("confidence".to_string(), json!(result.confidence));
                        obj.insert("samples_used".to_string(), json!(result.samples_used));
                        obj.insert(
                            "disambiguation_applied".to_string(),
                            json!(result.disambiguation_applied),
                        );
                        if let Some(rule) = &result.disambiguation_rule {
                            obj.insert("disambiguation_rule".to_string(), json!(rule));
                        }
                        let votes: Vec<serde_json::Value> = result
                            .vote_distribution
                            .iter()
                            .filter(|(_, f)| *f >= 0.01)
                            .map(|(l, f)| json!({"label": l, "fraction": f}))
                            .collect();
                        obj.insert("vote_distribution".to_string(), json!(votes));
                        println!(
                            "{}",
                            serde_json::to_string_pretty(&serde_json::Value::Object(obj))?
                        );
                    }
                    OutputFormat::Csv => {
                        println!(
                            "{},{:.4},{}",
                            result.label, result.confidence, result.samples_used
                        );
                    }
                }
            }
            _ => {
                eprintln!("Column mode is currently only supported with --model-type char-cnn");
                std::process::exit(1);
            }
        }
        return Ok(());
    }

    // Row mode: classify each value independently
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
        ModelType::Tiered => {
            let classifier = finetype_model::TieredClassifier::load(&model)?;
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
        ModelType::Tiered => {
            use finetype_model::{TieredTrainer, TieredTrainingConfig};

            let config = TieredTrainingConfig {
                batch_size,
                epochs,
                learning_rate: 1e-3,
                max_seq_length: 128,
                embed_dim: 32,
                num_filters: 64,
                hidden_dim: 128,
                weight_decay: 1e-4,
                tier2_min_types: 1,
            };

            eprintln!("Training Tiered models (Tier 0 → Tier 1 → Tier 2)");
            eprintln!("Training config: {:?}", config);

            let trainer = TieredTrainer::new(config);
            let report = trainer.train_all(&taxonomy, &samples, &output)?;
            eprintln!("{}", report);
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

    defs.sort_by_key(|(k, _)| (*k).clone());

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
// CHECK — Validate generator ↔ taxonomy alignment
// ═══════════════════════════════════════════════════════════════════════════════

fn cmd_check(
    taxonomy_path: PathBuf,
    samples: usize,
    seed: u64,
    priority: Option<u8>,
    verbose: bool,
    output: OutputFormat,
) -> Result<()> {
    eprintln!("Loading taxonomy from {:?}", taxonomy_path);
    let taxonomy = load_taxonomy(&taxonomy_path)?;
    eprintln!("Loaded {} definitions", taxonomy.len());

    let checker = Checker::new(samples).with_seed(seed);
    eprintln!(
        "Checking {} samples per definition (seed={})...",
        samples, seed
    );

    let report = checker.run(&taxonomy);

    match output {
        OutputFormat::Plain => {
            print!("{}", format_report(&report, verbose));
        }
        OutputFormat::Json => {
            let results: Vec<serde_json::Value> = report
                .results
                .iter()
                .filter(|r| priority.map(|p| r.release_priority >= p).unwrap_or(true))
                .map(|r| {
                    let mut obj = serde_json::Map::new();
                    obj.insert("key".to_string(), json!(r.key));
                    obj.insert("domain".to_string(), json!(r.domain));
                    obj.insert("generator_exists".to_string(), json!(r.generator_exists));
                    obj.insert("samples_generated".to_string(), json!(r.samples_generated));
                    obj.insert("samples_passed".to_string(), json!(r.samples_passed));
                    obj.insert("samples_failed".to_string(), json!(r.samples_failed));
                    obj.insert("pass_rate".to_string(), json!(r.pass_rate()));
                    obj.insert("has_pattern".to_string(), json!(r.has_pattern));
                    obj.insert("release_priority".to_string(), json!(r.release_priority));
                    obj.insert("passed".to_string(), json!(r.passed()));
                    if !r.failures.is_empty() {
                        let failures: Vec<serde_json::Value> = r
                            .failures
                            .iter()
                            .map(|f| {
                                json!({
                                    "sample": f.sample,
                                    "reason": format!("{}", f.reason),
                                })
                            })
                            .collect();
                        obj.insert("failures".to_string(), json!(failures));
                    }
                    serde_json::Value::Object(obj)
                })
                .collect();

            let summary = json!({
                "total_definitions": report.total_definitions,
                "generators_found": report.generators_found,
                "generators_missing": report.generators_missing,
                "fully_passing": report.fully_passing,
                "has_failures": report.has_failures,
                "no_pattern": report.no_pattern,
                "total_samples": report.total_samples,
                "total_passed": report.total_passed,
                "total_failed": report.total_failed,
                "pass_rate": report.pass_rate(),
                "all_passed": report.all_passed(),
                "results": results,
            });

            println!("{}", serde_json::to_string_pretty(&summary)?);
        }
        OutputFormat::Csv => {
            println!("key,domain,generator_exists,samples_generated,samples_passed,samples_failed,pass_rate,has_pattern,priority,passed");
            for r in &report.results {
                if priority.map(|p| r.release_priority >= p).unwrap_or(true) {
                    println!(
                        "\"{}\",\"{}\",{},{},{},{},{:.4},{},{},{}",
                        r.key,
                        r.domain,
                        r.generator_exists,
                        r.samples_generated,
                        r.samples_passed,
                        r.samples_failed,
                        r.pass_rate(),
                        r.has_pattern,
                        r.release_priority,
                        r.passed(),
                    );
                }
            }
        }
    }

    // Exit non-zero if checks failed
    if !report.all_passed() {
        std::process::exit(1);
    }

    Ok(())
}

// ═══════════════════════════════════════════════════════════════════════════════
// EVAL — Evaluate model accuracy on a test set
// ═══════════════════════════════════════════════════════════════════════════════

fn cmd_eval(
    data: PathBuf,
    model: PathBuf,
    _taxonomy_path: PathBuf,
    model_type: ModelType,
    top_confusions: usize,
    output: OutputFormat,
) -> Result<()> {
    use finetype_model::{CharClassifier, ClassificationResult};
    use std::collections::HashMap;

    eprintln!("Loading test data from {:?}", data);
    let file = std::fs::File::open(&data)?;
    let reader = std::io::BufReader::new(file);

    let mut test_samples: Vec<(String, String)> = Vec::new();
    for line in reader.lines() {
        let line = line?;
        if line.is_empty() {
            continue;
        }
        let record: serde_json::Value = serde_json::from_str(&line)?;
        let text = record["text"].as_str().unwrap_or("").to_string();
        let label = record["classification"].as_str().unwrap_or("").to_string();
        test_samples.push((text, label));
    }
    eprintln!("Loaded {} test samples", test_samples.len());

    // Run inference
    eprintln!("Loading model from {:?}", model);
    let mut predictions: Vec<ClassificationResult> = Vec::new();

    match model_type {
        ModelType::CharCnn => {
            let classifier = CharClassifier::load(&model)?;
            eprintln!("Running inference...");

            // Batch inference for efficiency
            let batch_size = 128;
            let texts: Vec<String> = test_samples.iter().map(|(t, _)| t.clone()).collect();
            for chunk in texts.chunks(batch_size) {
                let batch_results = classifier.classify_batch(chunk)?;
                predictions.extend(batch_results);
            }
        }
        ModelType::Transformer => {
            let classifier = Classifier::load(&model)?;
            eprintln!("Running inference...");

            let batch_size = 32;
            let texts: Vec<String> = test_samples.iter().map(|(t, _)| t.clone()).collect();
            for chunk in texts.chunks(batch_size) {
                let batch_results = classifier.classify_batch(chunk)?;
                predictions.extend(batch_results);
            }
        }
        ModelType::Tiered => {
            let classifier = finetype_model::TieredClassifier::load(&model)?;
            eprintln!("Running tiered inference...");

            let batch_size = 128;
            let texts: Vec<String> = test_samples.iter().map(|(t, _)| t.clone()).collect();
            for chunk in texts.chunks(batch_size) {
                let batch_results = classifier.classify_batch(chunk)?;
                predictions.extend(batch_results);
            }
        }
    }

    eprintln!("Computing metrics...");

    // Compute metrics
    let mut correct = 0usize;
    let mut top3_correct = 0usize;
    let total = test_samples.len();

    // Per-class counts: true_positives, false_positives, false_negatives
    let mut tp: HashMap<String, usize> = HashMap::new();
    let mut fp: HashMap<String, usize> = HashMap::new();
    let mut fn_: HashMap<String, usize> = HashMap::new();

    // Confusion pairs: (actual, predicted) -> count
    let mut confusion: HashMap<(String, String), usize> = HashMap::new();

    // Confidence distribution
    let mut confidence_correct: Vec<f32> = Vec::new();
    let mut confidence_wrong: Vec<f32> = Vec::new();

    for (i, ((_text, actual), pred)) in test_samples.iter().zip(predictions.iter()).enumerate() {
        let predicted = &pred.label;

        if predicted == actual {
            correct += 1;
            confidence_correct.push(pred.confidence);
            *tp.entry(actual.clone()).or_default() += 1;
        } else {
            confidence_wrong.push(pred.confidence);
            *fp.entry(predicted.clone()).or_default() += 1;
            *fn_.entry(actual.clone()).or_default() += 1;
            *confusion
                .entry((actual.clone(), predicted.clone()))
                .or_default() += 1;
        }

        // Top-3 accuracy
        let mut scores = pred.all_scores.clone();
        scores.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        let top3_labels: Vec<&str> = scores.iter().take(3).map(|(l, _)| l.as_str()).collect();
        if top3_labels.contains(&actual.as_str()) {
            top3_correct += 1;
        }

        // Progress
        if (i + 1) % 1000 == 0 {
            eprint!("\r  Processed {}/{}...", i + 1, total);
        }
    }
    eprintln!();

    let accuracy = correct as f64 / total as f64;
    let top3_accuracy = top3_correct as f64 / total as f64;

    let avg_confidence_correct = if confidence_correct.is_empty() {
        0.0
    } else {
        confidence_correct.iter().sum::<f32>() / confidence_correct.len() as f32
    };
    let avg_confidence_wrong = if confidence_wrong.is_empty() {
        0.0
    } else {
        confidence_wrong.iter().sum::<f32>() / confidence_wrong.len() as f32
    };

    // Collect all classes
    let mut all_classes: Vec<String> = tp
        .keys()
        .chain(fp.keys())
        .chain(fn_.keys())
        .cloned()
        .collect::<std::collections::HashSet<_>>()
        .into_iter()
        .collect();
    all_classes.sort();

    // Sort confusions by count
    let mut confusion_vec: Vec<((String, String), usize)> = confusion.into_iter().collect();
    confusion_vec.sort_by(|a, b| b.1.cmp(&a.1));

    match output {
        OutputFormat::Plain | OutputFormat::Csv => {
            println!("FineType Model Evaluation");
            println!("{}", "=".repeat(60));
            println!();
            println!("OVERALL");
            println!("  Samples:        {}", total);
            println!(
                "  Accuracy:       {:.2}% ({}/{})",
                accuracy * 100.0,
                correct,
                total
            );
            println!(
                "  Top-3 Accuracy: {:.2}% ({}/{})",
                top3_accuracy * 100.0,
                top3_correct,
                total
            );
            println!(
                "  Avg confidence (correct):   {:.4}",
                avg_confidence_correct
            );
            println!("  Avg confidence (incorrect): {:.4}", avg_confidence_wrong);
            println!();

            // Per-class metrics
            println!("PER-CLASS METRICS");
            println!(
                "  {:50} {:>6} {:>6} {:>6} {:>8}",
                "class", "prec", "rec", "f1", "support"
            );
            println!("  {}", "-".repeat(80));

            let mut macro_precision = 0.0f64;
            let mut macro_recall = 0.0f64;
            let mut macro_f1 = 0.0f64;
            let mut n_classes = 0;

            for class in &all_classes {
                let t = *tp.get(class).unwrap_or(&0) as f64;
                let f_p = *fp.get(class).unwrap_or(&0) as f64;
                let f_n = *fn_.get(class).unwrap_or(&0) as f64;

                let precision = if t + f_p > 0.0 { t / (t + f_p) } else { 0.0 };
                let recall = if t + f_n > 0.0 { t / (t + f_n) } else { 0.0 };
                let f1 = if precision + recall > 0.0 {
                    2.0 * precision * recall / (precision + recall)
                } else {
                    0.0
                };
                let support = (t + f_n) as usize;

                if support > 0 {
                    println!(
                        "  {:50} {:>5.1}% {:>5.1}% {:>5.1}% {:>8}",
                        class,
                        precision * 100.0,
                        recall * 100.0,
                        f1 * 100.0,
                        support,
                    );
                    macro_precision += precision;
                    macro_recall += recall;
                    macro_f1 += f1;
                    n_classes += 1;
                }
            }

            if n_classes > 0 {
                println!("  {}", "-".repeat(80));
                println!(
                    "  {:50} {:>5.1}% {:>5.1}% {:>5.1}% {:>8}",
                    "macro avg",
                    (macro_precision / n_classes as f64) * 100.0,
                    (macro_recall / n_classes as f64) * 100.0,
                    (macro_f1 / n_classes as f64) * 100.0,
                    total,
                );
            }

            // Top confusions
            if !confusion_vec.is_empty() {
                println!();
                println!("TOP CONFUSIONS (actual -> predicted)");
                for ((actual, predicted), count) in confusion_vec.iter().take(top_confusions) {
                    println!("  {:>4}x  {} -> {}", count, actual, predicted);
                }
            }
        }
        OutputFormat::Json => {
            let per_class: Vec<serde_json::Value> = all_classes
                .iter()
                .filter_map(|class| {
                    let t = *tp.get(class).unwrap_or(&0) as f64;
                    let f_p = *fp.get(class).unwrap_or(&0) as f64;
                    let f_n = *fn_.get(class).unwrap_or(&0) as f64;
                    let support = (t + f_n) as usize;
                    if support == 0 {
                        return None;
                    }
                    let precision = if t + f_p > 0.0 { t / (t + f_p) } else { 0.0 };
                    let recall = if t + f_n > 0.0 { t / (t + f_n) } else { 0.0 };
                    let f1 = if precision + recall > 0.0 {
                        2.0 * precision * recall / (precision + recall)
                    } else {
                        0.0
                    };
                    Some(json!({
                        "class": class,
                        "precision": precision,
                        "recall": recall,
                        "f1": f1,
                        "support": support,
                    }))
                })
                .collect();

            let top_conf: Vec<serde_json::Value> = confusion_vec
                .iter()
                .take(top_confusions)
                .map(|((actual, predicted), count)| {
                    json!({
                        "actual": actual,
                        "predicted": predicted,
                        "count": count,
                    })
                })
                .collect();

            let result = json!({
                "total_samples": total,
                "accuracy": accuracy,
                "top3_accuracy": top3_accuracy,
                "correct": correct,
                "avg_confidence_correct": avg_confidence_correct,
                "avg_confidence_wrong": avg_confidence_wrong,
                "per_class": per_class,
                "top_confusions": top_conf,
            });
            println!("{}", serde_json::to_string_pretty(&result)?);
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

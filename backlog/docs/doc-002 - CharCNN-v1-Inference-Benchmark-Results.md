---
id: doc-002
title: CharCNN v1 Inference Benchmark Results
type: other
created_date: '2026-02-10 12:23'
---
## Environment

- **Hardware**: x86_64 (Intel), CPU only, no GPU/Metal
- **Model**: char-cnn-v1 (331KB safetensors, 151 classes)
- **Binary**: finetype release build (LTO enabled, codegen-units=1)
- **Test data**: 151 synthetic types, 10 samples/label

## Results

### Model Load Time

Cold start (first run): **66ms**
Warm runs (cached): **25-30ms** average

### Single-Value Inference Latency (50 samples)

| Metric | Value |
|--------|-------|
| Min | 15ms |
| **p50** | **26ms** |
| **p95** | **41ms** |
| p99 | 50ms |
| Max | 54ms |
| Mean | 28.6ms |

Note: Latency includes process startup, model loading, tokenization, and inference. In-process latency (DuckDB extension, no startup overhead) would be significantly lower.

### Batch Throughput

| Batch Size | Time | Throughput |
|------------|------|-----------|
| 100 | 186ms | 537 values/sec |
| 1,000 | 1,362ms | 749 values/sec |
| 1,500 | 2,350ms | 651 values/sec |
| **10,717** | **17,362ms** | **617 values/sec** |

Throughput is approximately **600-750 values/sec** on CPU with CLI overhead. In-process (DuckDB extension) throughput would be higher since model loading is amortized.

### Memory Footprint

| Metric | Value |
|--------|-------|
| Maximum RSS | **8.5 MB** |
| Model size on disk | 331 KB |
| Labels mapping | 5.1 KB |

Very lightweight — the 8.5MB RSS includes the full Rust runtime, tokenizer, and model weights.

## Implications for DuckDB Extension

1. **Model load**: 25-30ms is acceptable for extension INSTALL/LOAD
2. **Per-value latency**: ~1.5ms in-process (estimated, removing CLI overhead) is fast enough for scalar functions
3. **Memory**: 8.5MB total is well within DuckDB extension norms
4. **Batch processing**: 600+ values/sec on CPU is reasonable for column profiling; larger batches could benefit from vectorized inference

## Comparison Points

- v1 model: 331KB, 151 classes, 89.8% accuracy
- Expected v2 model: similar size, improved accuracy from generator fixes
- Tiered models: may increase total model size 3-5x but each tier is smaller and faster

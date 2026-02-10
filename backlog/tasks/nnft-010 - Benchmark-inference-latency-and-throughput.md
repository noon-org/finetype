---
id: NNFT-010
title: Benchmark inference latency and throughput
status: Done
assignee:
  - '@nightingale'
created_date: '2026-02-10 05:31'
updated_date: '2026-02-10 12:24'
labels:
  - performance
  - benchmark
milestone: 'Phase 3: Build & Train'
dependencies:
  - NNFT-009
references:
  - crates/finetype-model/src/inference.rs
  - crates/finetype-cli/src/main.rs
priority: medium
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Measure single-value inference latency and batch throughput for the CharCNN model. Establish baseline performance numbers for: model load time, single inference (p50/p95/p99), batch inference (100/1000/10000 values), and memory footprint. These numbers inform DuckDB extension feasibility and expected UX.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 Model load time measured (cold start)
- [x] #2 Single-value inference latency measured: p50, p95, p99
- [x] #3 Batch throughput measured at 100, 1,000, and 10,000 values
- [x] #4 Memory footprint measured during inference
- [x] #5 Results documented in a benchmark report or eval output
- [x] #6 Benchmarks run on CPU (and Metal if available)
<!-- AC:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
- Benchmarked using char-cnn-v1 (331KB, 151 classes) on x86_64 CPU
- Model load: 66ms cold, 25-30ms warm
- Single inference: p50=26ms, p95=41ms (includes CLI startup overhead)
- Batch throughput: 600-750 values/sec on CPU (CLI mode)
- Memory: 8.5MB RSS peak (very lightweight)
- 10K batch test: 10,717 values in 17.4s
- No Metal/GPU available on this machine (x86_64)
- Results documented in doc-002
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Benchmarked CharCNN v1 inference performance on x86_64 CPU. Results documented in doc-002.

Key findings:
- **Model load**: 66ms cold start, 25-30ms warm (cached)
- **Single-value latency**: p50=26ms, p95=41ms, p99=50ms (includes CLI process startup)
- **Batch throughput**: 600-750 values/sec (CLI mode, x86_64 CPU)
- **Memory**: 8.5MB peak RSS — very lightweight for a 151-class model
- **10K batch**: 10,717 values processed in 17.4 seconds

DuckDB extension implications: in-process latency estimated at ~1.5ms/value (no CLI startup overhead), model load acceptable for extension INSTALL/LOAD, memory footprint well within extension norms.

Note: Only CPU benchmarks available (no Metal on x86_64). v2 benchmarks will follow after NNFT-009 training completes.
<!-- SECTION:FINAL_SUMMARY:END -->

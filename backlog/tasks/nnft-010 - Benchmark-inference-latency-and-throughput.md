---
id: NNFT-010
title: Benchmark inference latency and throughput
status: To Do
assignee: []
created_date: '2026-02-10 05:31'
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
- [ ] #1 Model load time measured (cold start)
- [ ] #2 Single-value inference latency measured: p50, p95, p99
- [ ] #3 Batch throughput measured at 100, 1,000, and 10,000 values
- [ ] #4 Memory footprint measured during inference
- [ ] #5 Results documented in a benchmark report or eval output
- [ ] #6 Benchmarks run on CPU (and Metal if available)
<!-- AC:END -->

# Changelog

All notable changes to MAR 1.0 will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [1.0.0-alpha] - 2026-07-29

### Added
- Initial open-source release
- 347M parameter decoder-only transformer with GQA, SwiGLU, RoPE, RMSNorm
- Urdu-English BPE tokenizer trainer
- PyTorch training pipeline for English pretraining (Phase 1A)
- GGUF export for llama.cpp inference
- Rust inference server (llama-cpp-2 backend)
- Axum API server with OpenAI-compatible chat endpoints
- Tauri 2.x desktop shell with system tray
- SolidJS 1.9 frontend with fine-grained reactivity
- Pakistan-domain knowledge base (10 skills domains)
- Urdu NLP pipeline (normalization, Roman Urdu, code-switching)
- Citation safety service for grounded responses
- Docker Compose infrastructure (PostgreSQL, DragonflyDB, Qdrant, Meilisearch)
- Prometheus + OpenTelemetry observability
- CI/CD with GitHub Actions (lint, test, build, release)
- Cross-platform support (Linux, Windows, macOS)

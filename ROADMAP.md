# MAR 1.0 Roadmap

## Vision

Build the fastest, most helpful AI assistant for Pakistan — with world-class
multilingual conversation, deep domain expertise, and safe, cited answers.

## Phases

### Phase 1A: English Foundation Model (Current)
**Target:** 350M parameter English-only model

- [x] Model architecture (GQA, SwiGLU, RoPE, RMSNorm)
- [x] BPE tokenizer trainer
- [x] Training pipeline (PyTorch, AdamW, cosine LR)
- [x] GGUF export → llama.cpp inference
- [x] Full-stack platform (Rust API + Tauri Desktop + SolidJS Frontend)
- [ ] Train on 10B tokens of FineWeb
- [ ] Achieve perplexity benchmark comparable to TinyLlama
- [ ] Release GGUF weights

### Phase 1B: Urdu Bilingual Extension
**Target:** Extend 350M model with Urdu capability

- [ ] Extend tokenizer vocabulary (32K → 64K with Urdu Unicode block)
- [ ] Prepare Urdu-English corpus (news, Wikipedia, literature, Roman Urdu)
- [ ] Continue pretraining with Urdu-English mixed data
- [ ] Evaluate Urdu perplexity and benchmark
- [ ] Release bilingual GGUF weights

### Phase 2: 3B Parameter Model
**Target:** 3B parameter bilingual model

- [ ] Scale architecture: 32 layers, 3200 hidden, 32 heads
- [ ] Multi-GPU training with FSDP
- [ ] Train on 500B+ tokens (English + Urdu)
- [ ] Achieve competitive benchmarks with Llama 3.2 3B
- [ ] Release 3B bilingual GGUF weights

### Phase 3: 8B+ MoE Model
**Target:** 8B total (2B active) Mixture-of-Experts model

- [ ] Design MoE architecture with 8 experts
- [ ] Implement MoE training pipeline
- [ ] Train on 1T+ tokens
- [ ] Optimize CPU inference with quantization
- [ ] Release 8B MoE GGUF weights

### Platform Improvements

- [ ] Web search integration
- [ ] PDF/image understanding
- [ ] Code execution sandbox
- [ ] Mobile app (Tauri Mobile)
- [ ] Plugin system for community skills
- [ ] RLHF/DPO alignment training

## How to Contribute

See [CONTRIBUTING.md](CONTRIBUTING.md) for how to get involved at each phase.

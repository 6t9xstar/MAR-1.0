#!/bin/bash
# Run MAR 1.0 Phase 1A training on a single GPU

export CUDA_VISIBLE_DEVICES=0
export OMP_NUM_THREADS=8
export TOKENIZERS_PARALLELISM=false

python train.py \
    --config configs/350m.yaml \
    --data-dir data/fineweb_raw \
    --output-dir checkpoints/mar_350m \
    --run-name mar_350m_english_v1 \
    --wandb

# Without wandb:
# python train.py \
#     --config configs/350m.yaml \
#     --data-dir data/fineweb_raw \
#     --output-dir checkpoints/mar_350m \
#     --run-name mar_350m_english_v1

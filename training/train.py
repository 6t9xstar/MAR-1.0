import os
import math
import time
import argparse

import yaml
import torch
from torch.utils.data import DataLoader

from model.configuration_mar import MARConfig
from model.modeling_mar import MARForCausalLM
from model.tokenization_mar import MARTokenizer
from data.dataset import TextFileDataset

try:
    import wandb
    WANDB_AVAILABLE = True
except ImportError:
    WANDB_AVAILABLE = False


def get_lr(it, config):
    warmup_iters = config["training"]["warmup_steps"]
    lr_decay_iters = config["training"]["total_steps"]
    min_lr = config["training"]["learning_rate"] * 0.1

    if it < warmup_iters:
        return config["training"]["learning_rate"] * it / warmup_iters
    if it > lr_decay_iters:
        return min_lr

    decay_ratio = (it - warmup_iters) / (lr_decay_iters - warmup_iters)
    coeff = 0.5 * (1.0 + math.cos(math.pi * decay_ratio))
    return min_lr + coeff * (config["training"]["learning_rate"] - min_lr)


def train():
    parser = argparse.ArgumentParser()
    parser.add_argument("--config", type=str, required=True)
    parser.add_argument("--data-dir", type=str, required=True)
    parser.add_argument("--output-dir", type=str, default="checkpoints/mar_350m")
    parser.add_argument("--run-name", type=str, default="mar_350m")
    parser.add_argument("--tokenizer-path", type=str, default="tokenizer/mar_32k")
    parser.add_argument("--resume-from", type=str, default=None)
    parser.add_argument("--wandb", action="store_true", default=False)
    args = parser.parse_args()

    with open(args.config) as f:
        config = yaml.safe_load(f)

    os.makedirs(args.output_dir, exist_ok=True)
    os.makedirs(os.path.join(args.output_dir, "logs"), exist_ok=True)

    device = torch.device("cuda" if torch.cuda.is_available() else "cpu")
    print(f"Using device: {device}")

    model_config = MARConfig(**config["model"])
    model = MARForCausalLM(model_config)
    model = model.to(device)

    total_params = sum(p.numel() for p in model.parameters())
    trainable_params = sum(p.numel() for p in model.parameters() if p.requires_grad)
    print(f"Total parameters: {total_params:,}")
    print(f"Trainable parameters: {trainable_params:,}")

    if model_config.tie_word_embeddings:
        non_emb = total_params - model.model.embed_tokens.weight.numel()
        print(f"Tied embeddings: {model.model.embed_tokens.weight.numel():,} shared")
        print(f"Actual unique parameters: {non_emb:,}")

    if args.wandb and WANDB_AVAILABLE:
        wandb.init(project="mar-1.0", name=args.run_name, config=config)

    tokenizer = MARTokenizer.from_pretrained(args.tokenizer_path)
    train_data_path = os.path.join(args.data_dir, "fineweb_text.txt")
    dataset = TextFileDataset(
        file_path=train_data_path,
        tokenizer=tokenizer,
        seq_length=config["training"]["sequence_length"],
    )

    total_tokens = config["training"]["total_tokens"]
    batch_size = config["training"]["batch_size"]
    seq_length = config["training"]["sequence_length"]
    tokens_per_step = batch_size * seq_length
    total_steps = total_tokens // tokens_per_step

    config["training"]["total_steps"] = total_steps
    print(f"Total training steps: {total_steps} ({total_tokens // 1e9:.1f}B tokens)")

    dataloader = DataLoader(
        dataset,
        batch_size=batch_size,
        num_workers=0,
        pin_memory=True,
    )
    data_iter = iter(dataloader)

    optimizer = torch.optim.AdamW(
        model.parameters(),
        lr=config["training"]["learning_rate"],
        betas=(config["training"]["beta1"], config["training"]["beta2"]),
        weight_decay=config["training"]["weight_decay"],
    )

    scaler = torch.cuda.amp.GradScaler() if device.type == "cuda" else None
    dtype = torch.bfloat16 if device.type == "cuda" and torch.cuda.is_bf16_supported() else torch.float16

    step = 0
    accum_loss = 0.0
    accum_tokens = 0
    start_time = time.time()
    tokens_processed = 0

    model.train()
    while step < total_steps:
        try:
            batch = next(data_iter)
        except StopIteration:
            data_iter = iter(dataloader)
            batch = next(data_iter)

        lr = get_lr(step, config)

        input_ids = batch["input_ids"].to(device)
        labels = batch["labels"].to(device)
        batch_tokens = input_ids.numel()

        with torch.cuda.amp.autocast(enabled=device.type == "cuda", dtype=dtype):
            outputs = model(input_ids=input_ids, labels=labels)
            loss = outputs.loss

        if scaler is not None:
            scaler.scale(loss).backward()
            scaler.unscale_(optimizer)
            torch.nn.utils.clip_grad_norm_(model.parameters(), config["training"]["grad_clip"])
            scaler.step(optimizer)
            scaler.update()
        else:
            loss.backward()
            torch.nn.utils.clip_grad_norm_(model.parameters(), config["training"]["grad_clip"])
            optimizer.step()

        optimizer.zero_grad()

        for param_group in optimizer.param_groups:
            param_group["lr"] = lr

        accum_loss += loss.item()
        accum_tokens += batch_tokens
        tokens_processed += batch_tokens
        step += 1

        if step % config["training"]["logging_steps"] == 0:
            elapsed = time.time() - start_time
            avg_loss = accum_loss / config["training"]["logging_steps"]
            tok_per_sec = accum_tokens / elapsed

            print(f"step {step}/{total_steps} | loss {avg_loss:.4f} | lr {lr:.2e} | tok/s {tok_per_sec:.0f} | tokens {tokens_processed / 1e6:.1f}M")

            if args.wandb and WANDB_AVAILABLE:
                wandb.log({
                    "loss": avg_loss,
                    "lr": lr,
                    "tok_per_sec": tok_per_sec,
                    "tokens": tokens_processed,
                    "perplexity": math.exp(avg_loss),
                }, step=step)

            accum_loss = 0.0
            accum_tokens = 0
            start_time = time.time()

        if step % config["training"]["save_steps"] == 0:
            checkpoint_dir = os.path.join(args.output_dir, f"step_{step}")
            os.makedirs(checkpoint_dir, exist_ok=True)
            model.save_pretrained(checkpoint_dir)
            model.config.save_pretrained(checkpoint_dir)
            print(f"Checkpoint saved to {checkpoint_dir}")

    final_dir = os.path.join(args.output_dir, "final")
    model.save_pretrained(final_dir)
    model.config.save_pretrained(final_dir)
    print(f"Training complete! Final model saved to {final_dir}")


if __name__ == "__main__":
    train()

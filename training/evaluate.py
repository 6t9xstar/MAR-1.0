import math
import torch
from torch.utils.data import DataLoader
from typing import Tuple

from model.configuration_mar import MARConfig
from model.modeling_mar import MARForCausalLM


@torch.no_grad()
def evaluate_perplexity(model: MARForCausalLM, dataloader: DataLoader, device: torch.device, max_batches: int = None) -> Tuple[float, float]:
    model.eval()
    total_loss = 0.0
    total_tokens = 0
    batches = 0

    for batch in dataloader:
        input_ids = batch["input_ids"].to(device)
        labels = batch["labels"].to(device)

        outputs = model(input_ids=input_ids, labels=labels)
        loss = outputs.loss

        total_loss += loss.item() * input_ids.numel()
        total_tokens += input_ids.numel()
        batches += 1

        if max_batches and batches >= max_batches:
            break

    avg_loss = total_loss / total_tokens
    perplexity = math.exp(avg_loss)
    model.train()
    return perplexity, avg_loss


@torch.no_grad()
def generate_text(model: MARForCausalLM, tokenizer, prompt: str, max_new_tokens: int = 100, temperature: float = 0.8, device: torch.device = "cpu") -> str:
    model.eval()
    input_ids = tokenizer.encode(prompt, return_tensors="pt").to(device)

    for _ in range(max_new_tokens):
        outputs = model(input_ids=input_ids)
        logits = outputs.logits[:, -1, :]
        probs = torch.softmax(logits / temperature, dim=-1)
        next_token = torch.multinomial(probs, num_samples=1)
        input_ids = torch.cat([input_ids, next_token], dim=-1)

        if next_token.item() == tokenizer.eos_token_id:
            break

    generated = tokenizer.decode(input_ids[0], skip_special_tokens=True)
    model.train()
    return generated

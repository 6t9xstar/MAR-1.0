import os
import json
from typing import Optional

import fire
from tokenizers import Tokenizer, models, trainers, pre_tokenizers, decoders, processors
from tokenizers.normalizers import NFC


def train_tokenizer(
    output_dir: str,
    vocab_size: int = 32768,
    corpus_files: Optional[list[str]] = None,
    min_frequency: int = 2,
) -> str:
    os.makedirs(output_dir, exist_ok=True)

    tokenizer = Tokenizer(models.BPE(unk_token="<|unk|>"))
    tokenizer.normalizer = NFC()
    tokenizer.pre_tokenizer = pre_tokenizers.ByteLevel(add_prefix_space=False)
    tokenizer.decoder = decoders.ByteLevel()
    tokenizer.post_processor = processors.ByteLevel(trim_offsets=True)

    trainer = trainers.BpeTrainer(
        vocab_size=vocab_size,
        min_frequency=min_frequency,
        special_tokens=[
            "<|pad|>",
            "<|endoftext|>",
            "<|unk|>",
            "<|bos|>",
            "<|eos|>",
            "<|system|>",
            "<|user|>",
            "<|assistant|>",
        ],
        show_progress=True,
        initial_alphabet=pre_tokenizers.ByteLevel.alphabet(),
    )

    if corpus_files:
        print(f"Training tokenizer on {len(corpus_files)} files...")
        tokenizer.train(files=corpus_files, trainer=trainer)
    else:
        print("No corpus files provided, training on sample data...")
        tokenizer.train_from_iterator(
            [
                "The quick brown fox jumps over the lazy dog.",
                "MAR 1.0 is a helpful AI assistant for Pakistan.",
                "Hello, how can I help you today?",
                "Artificial intelligence and machine learning are transforming the world.",
            ],
            trainer=trainer,
        )

    tokenizer_path = os.path.join(output_dir, "tokenizer.json")
    tokenizer.save(tokenizer_path)
    print(f"Tokenizer saved to {tokenizer_path}")
    print(f"Vocab size: {tokenizer.get_vocab_size()}")

    config = {
        "vocab_size": tokenizer.get_vocab_size(),
        "pad_token": "<|pad|>",
        "bos_token": "<|endoftext|>",
        "eos_token": "<|endoftext|>",
        "unk_token": "<|unk|>",
    }
    config_path = os.path.join(output_dir, "tokenizer_config.json")
    with open(config_path, "w") as f:
        json.dump(config, f, indent=2)
    print(f"Tokenizer config saved to {config_path}")

    return tokenizer_path


if __name__ == "__main__":
    fire.Fire(train_tokenizer)

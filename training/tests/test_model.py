import torch
import pytest

from model.configuration_mar import MARConfig
from model.modeling_mar import MARForCausalLM


def make_config(vocab_size=32000, hidden_size=256, num_hidden_layers=2,
                num_attention_heads=4, num_key_value_heads=2, intermediate_size=512,
                tie_word_embeddings=True):
    return MARConfig(
        vocab_size=vocab_size,
        hidden_size=hidden_size,
        num_hidden_layers=num_hidden_layers,
        num_attention_heads=num_attention_heads,
        num_key_value_heads=num_key_value_heads,
        intermediate_size=intermediate_size,
        max_position_embeddings=2048,
        tie_word_embeddings=tie_word_embeddings,
    )


def test_model_creation():
    config = make_config()
    model = MARForCausalLM(config)
    assert model is not None
    params = sum(p.numel() for p in model.parameters())
    assert params > 0


def test_forward_pass():
    config = make_config()
    model = MARForCausalLM(config)
    model.eval()

    input_ids = torch.randint(0, 32000, (2, 128))
    labels = input_ids.clone()

    with torch.no_grad():
        outputs = model(input_ids=input_ids, labels=labels)

    assert outputs.loss is not None
    assert outputs.logits is not None
    assert outputs.logits.shape == (2, 128, 32000)


def test_gqa_kv_heads():
    config = make_config(num_attention_heads=8, num_key_value_heads=2)
    model = MARForCausalLM(config)
    ratio = model.config.num_attention_heads // model.config.num_key_value_heads
    assert ratio == 4


def test_tied_embeddings():
    config = make_config(tie_word_embeddings=True)
    model = MARForCausalLM(config)
    assert model.lm_head.weight.data_ptr() == model.model.embed_tokens.weight.data_ptr()


def test_gradient_flow():
    config = make_config()
    model = MARForCausalLM(config)
    model.train()

    input_ids = torch.randint(0, 32000, (2, 64))
    labels = input_ids.clone()

    outputs = model(input_ids=input_ids, labels=labels)
    outputs.loss.backward()

    has_grad = any(param.grad is not None for param in model.parameters())
    assert has_grad, "No gradients flowed through the model"

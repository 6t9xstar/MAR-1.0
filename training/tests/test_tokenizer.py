import os
import sys
import shutil
import tempfile

sys.path.insert(0, os.path.join(os.path.dirname(__file__), ".."))

from model.tokenization_mar import MARTokenizer


def safe_print(msg: str):
    sys.stdout.buffer.write((msg + "\n").encode("utf-8", errors="replace"))
    sys.stdout.buffer.flush()


def test_tokenizer_creation():
    tokenizer = MARTokenizer()
    assert tokenizer is not None
    assert tokenizer.vocab_size > 0


def test_tokenizer_encode_decode():
    tokenizer = MARTokenizer()
    text = "Hello, how are you today?"
    tokens = tokenizer.encode(text)
    assert len(tokens) > 0


def test_tokenizer_special_tokens():
    tokenizer = MARTokenizer()
    assert tokenizer.pad_token_id is not None
    assert tokenizer.bos_token_id is not None
    assert tokenizer.eos_token_id is not None
    assert tokenizer.unk_token_id is not None


def test_tokenizer_save_load():
    tmp_dir = tempfile.mkdtemp(prefix="mar_tokenizer_")
    try:
        tokenizer = MARTokenizer()
        tokenizer.save_pretrained(tmp_dir)
        assert os.path.exists(os.path.join(tmp_dir, "tokenizer.json"))

        loaded = MARTokenizer.from_pretrained(tmp_dir)
        text = "Testing save and load functionality"
        original = tokenizer.encode(text)
        restored = loaded.encode(text)
        assert original == restored
    finally:
        shutil.rmtree(tmp_dir, ignore_errors=True)

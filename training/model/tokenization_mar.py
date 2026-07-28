import os
from typing import List, Optional, Tuple

from tokenizers import Tokenizer as HFTokenizer
from transformers import PreTrainedTokenizer


class MARTokenizer(PreTrainedTokenizer):
    def __init__(
        self,
        tokenizer_file: Optional[str] = None,
        pad_token: str = "<|pad|>",
        bos_token: str = "<|endoftext|>",
        eos_token: str = "<|endoftext|>",
        unk_token: str = "<|unk|>",
        **kwargs,
    ):
        if tokenizer_file is not None:
            self._tokenizer = HFTokenizer.from_file(tokenizer_file)
        else:
            self._tokenizer = HFTokenizer.from_pretrained("gpt2")

        super().__init__(
            pad_token=pad_token,
            bos_token=bos_token,
            eos_token=eos_token,
            unk_token=unk_token,
            **kwargs,
        )

    @property
    def vocab_size(self) -> int:
        return self._tokenizer.get_vocab_size()

    def _tokenize(self, text: str, **kwargs) -> List[str]:
        return self._tokenizer.encode(text).tokens

    def _convert_token_to_id(self, token: str) -> int:
        return self._tokenizer.token_to_id(token)

    def _convert_id_to_token(self, index: int) -> str:
        return self._tokenizer.id_to_token(index)

    def convert_tokens_to_ids(self, tokens: List[str]) -> List[int]:
        return [self._convert_token_to_id(t) for t in tokens]

    def convert_ids_to_tokens(self, ids: List[int], skip_special_tokens: bool = False) -> List[str]:
        return [self._convert_id_to_token(i) for i in ids]

    def get_vocab(self):
        return self._tokenizer.get_vocab()

    def save_vocabulary(self, save_directory: str, filename_prefix: Optional[str] = None) -> Tuple[str]:
        tokenizer_path = os.path.join(save_directory, "tokenizer.json")
        self._tokenizer.save(tokenizer_path)
        return (tokenizer_path,)

    @classmethod
    def from_pretrained(cls, pretrained_model_name_or_path, *inputs, **kwargs):
        tokenizer_file = os.path.join(pretrained_model_name_or_path, "tokenizer.json")
        if os.path.exists(tokenizer_file):
            kwargs["tokenizer_file"] = tokenizer_file
        return super().from_pretrained(pretrained_model_name_or_path, *inputs, **kwargs)

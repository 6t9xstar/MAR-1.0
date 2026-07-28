import os
import argparse
import torch
from safetensors.torch import load_file

from model.configuration_mar import MARConfig
from model.modeling_mar import MARForCausalLM


def convert_to_gguf(model_path: str, output_path: str, quantize: str = "q4_k_m"):
    print(f"Loading model from {model_path}...")
    config = MARConfig.from_pretrained(model_path)
    model = MARForCausalLM(config)
    model.eval()

    safetensors_path = os.path.join(model_path, "model.safetensors")
    if os.path.exists(safetensors_path):
        state_dict = load_file(safetensors_path)
    else:
        model_file = os.path.join(model_path, "pytorch_model.bin")
        state_dict = torch.load(model_file, map_location="cpu")

    model.load_state_dict(state_dict, strict=True)
    print(f"Model loaded: {sum(p.numel() for p in model.parameters()):,} params")

    hf_dir = os.path.join(model_path, "hf_model")
    os.makedirs(hf_dir, exist_ok=True)
    model.save_pretrained(hf_dir)
    config.save_pretrained(hf_dir)
    print(f"HuggingFace model saved to {hf_dir}")

    print(f"""
To convert to GGUF, run:

  pip install llama-cpp-python
  python -m llama_cpp.llama_cpp_convert \\
    --model-path {hf_dir} \\
    --output-path {output_path} \\
    --quantize {quantize}

Or using llama.cpp convert.py:

  python /path/to/llama.cpp/convert.py {hf_dir} \\
    --outfile {output_path} \\
    --quantize {quantize}

Then update inference/.env:
  MODEL_PATH={output_path}
  MODEL_ALIAS=mar-350m
""")

    return hf_dir


if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument("--model-path", required=True)
    parser.add_argument("--output-path", default="models/MAR-350M-Q4_K_M.gguf")
    parser.add_argument("--quantize", default="q4_k_m")
    args = parser.parse_args()
    convert_to_gguf(args.model_path, args.output_path, args.quantize)

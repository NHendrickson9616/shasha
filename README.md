# shasha hackathon starter

The key separation: a causal transformer scores *typed structural decisions*; a controller selects the applicable output head; deterministic code renders the result as Rust. This is a deliberately tiny teaching slice, not a copy of the full model.

## Run

```sh
cargo run -- "add two integers"
cargo test
```

`cargo run` selects CUDA if the binary was built with the CUDA feature and a device is available; otherwise it runs on CPU. To request a specific backend:

```sh
SHASHA_DEVICE=cpu cargo run
SHASHA_DEVICE=cuda cargo run --features cuda
```

CUDA requires an NVIDIA GPU with a working driver, a compatible CUDA toolkit including `nvcc` on `PATH`, and the Candle CUDA dependencies. An explicit CUDA request fails if the feature or device is unavailable; `auto` (the default) falls back to CPU at runtime. On the RTX 5080 used to test this starter, `/usr/bin/nvcc` is CUDA 12.0, which is too old for compute capability 12.0. The installed CUDA 13.0 toolkit works:

```sh
PATH=/usr/local/cuda/bin:/usr/bin:/bin CUDA_ROOT=/usr/local/cuda SHASHA_DEVICE=cuda cargo run --features cuda -- "add two integers"
```

The GPU was hidden inside the coding terminal's sandbox; the CUDA build and run above were verified outside it. A normal local terminal with GPU access does not need sandbox changes.

Cargo needs the dependencies in its cache or access to crates.io. The command-line argument is the input message. `src/tokenizer.rs` maps each UTF-8 byte directly to an ID from 0 to 255 (up to 114 bytes, reserving room for generated actions); the CLI shows those IDs and the selected action heads before printing the Rust function. This is a lossless byte tokenizer, **not a pretrained language tokenizer**. The model has randomly initialized weights, so the output does not understand English or produce useful logic. The no-parameter head scores `0`, `1`, `1 + 1`, and `1 - 1` as four equally available actions; both arithmetic expressions are still fixed templates, not independently generated operands. Equal availability does not imply equal random logits. There is **no task-specific mode**: the model can choose any of the supported function signatures and expressions, including addition. Every completed result is checked by `rustc` for syntax and types before being returned. If compilation fails, generation returns an error rather than presenting invalid Rust. This check does not prove that the function matches the prompt. Tests compile every structural branch and check that pointer-selected operands resolve to the right generated names. Running the CLI requires `rustc` on `PATH`.

## What is included

- `src/tokenizer.rs`: UTF-8 byte-to-ID encoding for a short text prompt.
- `src/model.rs`: prompt/action/Need embeddings, a causal attention block, routed structural heads (signature, expression, and name-character heads), and a dynamic symbol-pointer score over the controller's live parameter slots. Only the selected head runs.
- `src/controller.rs`: choices for zero, one, or two `i32` parameters, expressions including parallel `1 + 1` and `1 - 1` template actions, and parameter references with stable symbol IDs. After structure, a bounded name head emits lowercase characters for the function and parameters. The controller adds `n_` and a unique symbol-ID suffix so every spelling is a legal, distinct Rust identifier. Pointer actions select IDs, not their spellings.
- `src/lib.rs` and `src/validation.rs`: select head → score its choices → apply → render → check with `rustc`. No post-hoc action mask or task-specific override is used.

## Hackathon work to pick up

1. Train on paired prompt/action data. The current byte tokenizer encodes text but its randomly initialized embeddings do not give it language understanding. A pretrained tokenizer and backbone are alternatives for a larger experiment.
2. Build a supervised training loop over controller-derived actions. Decide how to train or initialize the English backbone; this starter has **no pretrained weights**.
3. Extend the controller and IR together (statements, calls, more types) before adding matching heads. Preserve each head's local action IDs when saving checkpoints.
4. Replace the starter's fixed two-slot learned symbol keys with declaration-state keys and richer scope tracking. The current pointer head scores only the one or two live `i32` parameters, not arbitrary Rust symbols; its keys are learned by slot and no weights have been trained.
5. Add evaluation against text generation at matched compute. Syntax validity alone does not establish better code quality.

Limits: one expression per function, at most two parameters, names limited to three generated lowercase letters plus a deterministic prefix/suffix; no caching, batching, checkpoint format, pretrained loading, or training. The chosen action is argmax; varying output across runs is expected with random initialization.

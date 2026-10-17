# MoPaPPa hackathon starter

A small, **independent** Rust/Candle starter based on `/home/moi/code/MoPaPPa`. The original repository is not required to build this one. It keeps the key separation: a causal transformer scores *typed structural decisions*; a controller restricts legal actions; deterministic code renders the result as Rust. This is a deliberately tiny teaching slice, not a copy of the full model.

## Run

```sh
cargo run
cargo test
```

CPU is the default. Cargo needs the dependencies in its cache or access to crates.io. The demo uses two arbitrary token IDs and **randomly initialized weights**. Its output shows that the parts connect; it does not understand English or generate useful programs.

## What is included

- `src/model.rs`: prompt/action/Need embeddings, one causal self-attention block, and separate output heads for root and expression decisions.
- `src/controller.rs`: two-step generation with stable candidate IDs, context-specific legal choices, and deterministic rendering of a function or block.
- `src/lib.rs`: the inference loop (score → mask → choose → apply → render).

## Hackathon work to pick up

1. Replace stand-in prompt IDs with a tokenizer and a real dataset of prompt/action pairs. Keep token IDs within `VOCAB_SIZE`.
2. Build a supervised training loop over controller-derived actions. Decide how to train or initialize the English backbone; this starter has **no pretrained weights**.
3. Extend the controller and IR together (symbols, parameters, statements, calls) before adding matching heads. Preserve action IDs when saving checkpoints.
4. Add dynamic symbol-pointer scoring and scope-aware legality. Do not treat symbol IDs as fixed vocabulary tokens.
5. Add evaluation against text generation at matched compute. Syntax validity alone does not establish better code quality.

Limits: one block, two decisions, no caching, batching, checkpoint format, tokenizer, pretrained loading, or training. The chosen action is argmax; varying output across runs is expected with random initialization.

# Shasha

Shasha is a small research/demo project exploring structured Rust code generation. It shows two complementary ideas:

- **Guided examples UI:** a React interface with three authored, deterministic pathways for learning how choices become readable Rust.
- **Structured generation prototype:** a tiny Rust/Candle model whose controller masks invalid actions before rendering a small Rust program.

The two demos are intentionally lightweight and currently run independently. The UI does not call the Rust model, and the Rust model is not trained to understand natural-language prompts.

## Explainer video

<video controls preload="metadata" poster="https://raw.githubusercontent.com/NHendrickson9616/shasha/main/manim_animation/ShashaExplainer.jpg" width="1280" src="https://raw.githubusercontent.com/NHendrickson9616/shasha/main/manim_animation/ShashaExplainer.mp4">
 <a href="https://raw.githubusercontent.com/NHendrickson9616/shasha/main/manim_animation/ShashaExplainer.mp4">Watch the explainer video</a>
</video>

The explainer shows how constrained semantic actions are rendered into valid
Rust.

## Quick start

### Guided examples UI

Requirements: Node.js and npm.

```sh
npm install
npm run dev
```

Open the local URL printed by Vite. To create a production build or preview it locally:

```sh
npm run build
npm run preview
```

The UI offers three fixed pathways:

1. **Grow a calculator** — addition, multiplication, and checked arithmetic.
2. **Search a collection** — finding even numbers, collecting matches, and stable de-duplication.
3. **Handle boundaries** — clamping, validating bounds, and using an inclusive range.

Each step exposes the selected function details beside the rendered Rust, and the Copy action copies that snippet to the clipboard.

### Rust generation prototype

Requirements: a Rust toolchain with Cargo. The default build targets CPU.

```sh
cargo run
cargo test
```

`cargo run` creates a randomly initialized model, scores two placeholder prompt IDs, and prints the controller's rendered result. Because the weights are random, the selected actions and output can vary between runs. The output demonstrates that the model, legality mask, controller, and renderer are connected; it is not a useful prediction.

## How the Rust prototype works

The inference loop is deliberately small:

```text
score → mask illegal actions → choose argmax → apply action → render
```

- `src/model.rs` defines the tiny causal transformer and its output heads.
- `src/controller.rs` defines the generation state, legal choices, stable action IDs, and rendering.
- `src/lib.rs` wires model scoring to controller steps and contains the integration tests.
- `src/main.rs` provides the command-line entry point.

The controller is the source of truth for what can be generated at each step. The model scores candidate actions, but cannot select an illegal one after masking.

## Current scope

This is a teaching slice, not a complete code-generation system. It currently has:

- one small generation shape and two controller decisions;
- placeholder prompt IDs instead of a tokenizer;
- randomly initialized weights, with no pretrained model;
- no training loop, dataset, checkpoint format, batching, or KV-cache;
- CPU-only defaults;
- no connection between the Rust prototype and the web UI.

Syntax validity alone is not evidence of code quality. Any future evaluation should compare against a text-generation baseline at matched compute.

## Directions for further work

1. Add a tokenizer and a real dataset of prompt/action pairs, keeping IDs within `VOCAB_SIZE`.
2. Train the model on controller-derived actions and define how an English backbone is initialized.
3. Extend the controller and intermediate representation together with symbols, parameters, statements, and calls.
4. Add scope-aware legality and dynamic symbol-pointer scoring rather than treating symbols as fixed vocabulary tokens.
5. Add checkpoint compatibility tests so action IDs remain stable across saved models.
6. Connect the UI to a real inference service once the generation interface is stable.

## Project structure

```text
src/       Rust model, controller, renderer, and tests
web/       React guided-examples interface
index.html Vite entry point and page metadata
```

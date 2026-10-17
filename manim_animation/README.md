# Shasha Manim animation

This folder contains a self-contained motion explainer based on
`../DEMO_BRIEF.md`. It turns the demo's central reversal into a visual story:
the model selects constrained semantic actions, and the surrounding system
renders valid Rust.

## Render

Create a virtual environment and install the pinned dependency:

```sh
cd manim_animation
python3.12 -m venv .venv
.venv/bin/pip install -r requirements.txt
```

Render the final 1080p film:

```sh
.venv/bin/manim shasha_explainer.py ShashaExplainer
```

For a quick iteration render:

```sh
.venv/bin/manim -ql shasha_explainer.py ShashaExplainer
```

The final video is written to
`media/videos/shasha_explainer/1080p30/shasha_explainer.mp4`.

## Storyboard

1. Show why unconstrained text generation creates a repair loop.
2. Reveal Shasha's division of responsibility: meaning on the left, syntax on
   the right.
3. Build the calculator through addition, multiplication, and checked
   arithmetic.
4. Expose the score, mask, choose, apply, render controller loop.
5. Reuse the same mechanism for collection search and close on the core idea.

The animation is intentionally silent. On-screen copy carries the complete
story and can also support a live voiceover.

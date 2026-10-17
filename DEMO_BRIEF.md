# Shasha demo brief

## The introduction we want judges to remember

**Shasha reverses the usual code-generation assumption: the model does not write
code. It chooses the meaning, and a deterministic system turns that meaning
into valid Rust.**

Most code-generation demos ask an AI to produce a long answer and hope the
syntax is correct. Shasha makes the boundary visible. The model selects from
legal, structured actions; the controller rejects anything illegal; and the
renderer supplies the punctuation, braces, names, and indentation.

Today, many LLM coding systems deal with invalid output by reprompting
themselves over and over again: generate a program, run or inspect it, describe
the error, and ask the model to try again. That can eventually produce a valid
program, but it spends additional compute and time repairing a problem that
could have been prevented at generation time. Shasha flips that trade-off by
constraining the action space before rendering, rather than relying on repeated
self-correction after the fact.

The result is a small but inspectable generation system. Judges can see not just
the code it produced, but the decisions that produced it and the constraints
that prevented an invalid program.

## The two-minute story

### 0:00–0:15 — Start with the old assumption

Say:

> “We usually think code generation means asking a model to write text. That
> makes syntax, structure, and reliability the model's problem. When the model
> gets it wrong, modern systems often reprompt themselves repeatedly until a
> valid program appears. That works, but it costs time and compute.”

Do not start on a title slide. Start with the Shasha interface already open,
with the three pathways visible.

### 0:15–0:30 — State the flip

Say:

> “Shasha flips that boundary. The model chooses the semantic action, while a
> constrained controller guarantees that only legal actions can be rendered as
> Rust.”

Point to the opening line in the UI:

> “The model chooses meaning. The system handles syntax.”

This is the central idea. Everything shown afterwards should prove it.

### 0:30–0:55 — Demonstrate the first decision

Choose **Grow a calculator** and select **Create a function that adds two
integers**.

Show the two columns together:

- **Function details:** function kind, inputs, result, and operation.
- **Rendered output:** the corresponding Rust function.

Say:

> “The output is not an unexplained block of generated text. We can inspect the
> semantic decisions beside the exact Rust they render to.”

Briefly point out that the controller owns the shape of the function and the
renderer owns syntax. Do not read every line aloud.

### 0:55–1:20 — Show composition, not autocomplete

Choose the suggested next message, **Please now add multiplication**.

Say:

> “The next request does not regenerate the whole answer from scratch. It
> extends the existing structure: the result becomes a tuple, and both
> operations reuse the same inputs.”

Then choose **Prevent integer overflow**.

Say:

> “Now the meaning changes again. The result becomes optional, and checked
> operations make failure explicit. The important point is that each change is
> represented as a small set of structured choices before it becomes text.”

The visible progression should be:

```text
function + addition
→ tuple + multiplication
→ optional result + checked operations
```

### 1:20–1:40 — Prove that this is not one hard-coded answer

Reset and choose **Search a collection**.

Show the first step, then continue to **Return every even number**. If time
allows, continue to **Remove duplicate values**.

Say:

> “The same mechanism handles a different pathway: searching, filtering, and
> stable de-duplication. The route is authored and deterministic for the demo,
> so every click is reproducible, but the underlying representation is the
> same: choose legal meaning, then render valid Rust.”

This is the selectivity and reliability proof. Do not claim that the current
web UI is connected to the Rust model; it is a deterministic guided interface,
while the Rust prototype independently demonstrates the model/controller loop.

### 1:40–1:55 — Explain the technical boundary

Say:

> “Underneath, the Rust prototype follows a small loop: score actions, mask
> illegal actions, choose the best legal action, apply it, and render. The
> controller is the source of truth for what can happen next.”

If asked, show the terminal output from `cargo run` or the relevant source
files after the timed demo. Keep the live presentation focused on the product
behaviour rather than implementation detail.

### 1:55–2:00 — Close on the reversal

Say:

> “Shasha is not trying to make a language model responsible for every
> character of a program. It makes the model responsible for meaning, and
> makes correctness a property of the system around it.”

Stop there. Leave the final rendered example visible for questions.

## What makes this creative

The creative claim is not “AI writes Rust faster.” That is a familiar
application. The creative claim is a changed division of responsibility:

| Conventional code generation | Shasha |
| --- | --- |
| The model emits text | The model selects structured actions |
| Syntax is a prediction problem | Syntax is supplied by a renderer |
| Invalid output triggers repeated self-reprompting | Illegal actions are masked before rendering |
| Extra repair attempts consume time and compute | Constraints prevent avoidable repair attempts |
| The answer is judged as a finished blob | The semantic trace is inspectable beside the answer |

The demo should make this reversal visible in under thirty seconds. If a judge
can only see the final code, we have missed the point.

## What is real, deterministic, and out of scope

Be precise when explaining the prototype:

- The web interface is real and interactive.
- Its three pathways are authored, fixed, and deterministic so the demo is
  repeatable.
- The UI displays the semantic trace and the rendered Rust together.
- The Rust prototype contains the model, legality mask, controller, renderer,
  and integration tests.
- The current UI does **not** call the Rust model.
- The Rust model uses randomly initialized weights and placeholder prompt IDs;
  it is a connected prototype, not a trained natural-language code generator.
- We should not claim production-level code quality, arbitrary prompt
  understanding, or model/UI integration that does not exist yet.

This honesty is part of the execution score: judges should understand exactly
which behaviour is demonstrated and which is the next engineering step.

## Demo setup and failure plan

Before presenting:

1. Run the web app and open the local URL.
2. Refresh once and leave the landing state visible.
3. Confirm the three pathway buttons and all suggested-message transitions.
4. Run `cargo test` and `cargo run` before the judging session.
5. Keep a second browser tab open at the landing state as a recovery path.
6. Keep a terminal ready for technical questions, not as the primary demo.

If the live UI fails, use the prepared screenshots or a short recording only as
an explanation of the same real flow. Do not replace the core demo with a
generic slide deck.

## Team roles

Assign one person to narrate and drive the interface.

- **Presenter:** delivers the two-minute story and controls the browser.
- **Systems lead:** explains the Rust model, legality mask, controller, and
  renderer.
- **Product/UI lead:** explains the guided pathways, deterministic scope, and
  why the trace is shown beside the output.

The presenter should not hand control to each teammate during the timed pitch.
Teammates answer questions after the close.

## Likely judge questions

**Is this just a template system?**  
The UI is intentionally a small authored teaching slice, not an open-ended
prompting product. Its purpose is to make the structured-generation boundary
observable. The Rust controller demonstrates the general score → mask → choose
→ apply → render loop.

**Why not let the model generate all the text?**  
Because syntax validity and semantic intent are different responsibilities.
Constraining the action space makes illegal structure impossible at the point of
selection and gives us an inspectable trace.

**Does the web UI use the Rust model?**  
Not currently. The two pieces are deliberately independent prototypes. The next
step is connecting the stable UI generation interface to a real inference
service once the model and tokenizer are ready.

**What would you build next?**  
Add a tokenizer and trained prompt/action dataset, extend the controller with
scope-aware symbols and statements, add checkpoint compatibility tests, and
connect the UI to the inference service.

## Presentation rule

Every screen and sentence should reinforce one question:

> **What if the model chose meaning, but the system—not the model—owned syntax?**

If a feature does not help answer that question in the two-minute demo, leave it
out.

---
theme: default
title: Shasha — The model chooses meaning
info: |
  A short visual story about constrained Rust generation.
author: Shasha
colorSchema: light
aspectRatio: 16/9
canvasWidth: 1280
transition: fade-out
mdc: true
download: false
exportFilename: ShashaSlides
---

<div class="slide-shell cover-slide">
  <div class="brand-lockup">
    <span class="brand-mark">S</span>
    <span>Shasha</span>
  </div>

  <div class="cover-copy">
    <p class="eyebrow">Structured Rust generation</p>
    <h1>The model chooses <em>meaning.</em><br>The system handles <strong>syntax.</strong></h1>
    <p class="lede">A visible boundary between semantic decisions and valid code.</p>
  </div>

  <div class="cover-footer">
    <span>Demo story</span>
    <span class="mono">01 / 09</span>
  </div>
</div>

<!--
We usually think code generation means asking a model to write text. Shasha
moves that boundary: the model chooses the semantic action, while a constrained
system turns that choice into valid Rust.
-->

---
layout: full
---

<div class="slide-shell">
  <header class="slide-header">
    <span class="section-number">01</span>
    <div>
      <p class="eyebrow">The old assumption</p>
      <h2>Ask a model to write every character.</h2>
    </div>
  </header>

  <div class="generation-flow">
    <div class="flow-node model-node">
      <small>Model</small>
      <b>Predict text</b>
      <span>syntax included</span>
    </div>
    <div class="flow-arrow"><span>→</span></div>
    <div class="code-window raw-window">
      <div class="code-title"><span>RUST</span><small>Raw output</small></div>
      <pre><code>fn calculate(a: i32, b: i32) {
    a +
}</code></pre>
    </div>
    <div class="flow-arrow"><span>→</span></div>
    <div class="flow-node reject-node">
      <small>Compiler</small>
      <b><i>×</i> Rejected</b>
      <span>invalid program</span>
    </div>
  </div>

  <div v-click class="repair-loop">
    <div class="loop-line"></div>
    <p class="mono">DESCRIBE ERROR&nbsp;&nbsp;→&nbsp;&nbsp;RE-PROMPT&nbsp;&nbsp;→&nbsp;&nbsp;TRY AGAIN</p>
    <div class="attempts">
      <span>Attempt 01</span><span>Attempt 02</span><span>Attempt 03</span>
    </div>
  </div>

  <p v-click class="bottom-statement">Repair can eventually work. Every extra attempt spends more time and compute.</p>
</div>

<!--
When the model owns syntax and structure, invalid output becomes a repair
problem. A modern system can inspect the error and reprompt itself repeatedly,
but that spends more inference on a failure that could be prevented earlier.
-->

---
layout: full
---

<div class="slide-shell boundary-slide">
  <header class="center-header">
    <p class="eyebrow">Shasha moves the boundary</p>
    <h2>Who owns what?</h2>
  </header>

  <div class="boundary-grid">
    <section>
      <p class="role red">The model</p>
      <h3>chooses<br><em>meaning.</em></h3>
      <p>Structured actions selected from the legal choices.</p>
      <div class="semantic-tag">Add(@1, @2)</div>
    </section>
    <div class="boundary-rule"><span>→</span></div>
    <section>
      <p class="role green">The system</p>
      <h3>handles<br><em>syntax.</em></h3>
      <p>Punctuation, braces, names, types, and indentation.</p>
      <div class="semantic-tag green-tag">left + right</div>
    </section>
  </div>

  <p class="bottom-statement">The controller guarantees that only legal actions reach the renderer.</p>
</div>

<!--
The model chooses the meaning. The controller owns what can happen next. The
renderer supplies the exact Rust syntax. Everything after this slide is proof
of that division of responsibility.
-->

---
layout: full
---

<div class="slide-shell inspect-slide">
  <header class="slide-header compact-header">
    <span class="section-number">02</span>
    <div>
      <p class="eyebrow">One decision, fully inspectable</p>
      <h2>Meaning and output stay side by side.</h2>
    </div>
  </header>

  <div class="request-bar"><span>Request</span>Create a function that adds two integers</div>

  <div class="workbench">
    <section class="trace-panel">
      <div class="panel-heading"><b>Function details</b><small>4 structured choices</small></div>
      <div class="trace-row"><i>✓</i><span><code>Function kind</code><small>Create one callable unit</small></span><b>Function</b></div>
      <div class="trace-row"><i>✓</i><span><code>Inputs</code><small>Accept two integer values</small></span><b>I32 × 2</b></div>
      <div class="trace-row"><i>✓</i><span><code>Result</code><small>Return one integer</small></span><b>I32</b></div>
      <div class="trace-row"><i>✓</i><span><code>Operation</code><small>Add the two input values</small></span><b>Add(@1, @2)</b></div>
    </section>

    <section class="code-window final-code">
      <div class="code-title"><span>RUST</span><small>Rendered output</small></div>
      <pre><code><span class="line-no">1</span> fn calculate(left: i32, right: i32) -> i32 {
<span class="line-no">2</span>     left + right
<span class="line-no">3</span> }</code></pre>
      <div class="code-meta"><span>Names used</span><code>@0 → calculate&nbsp;&nbsp; @1 → left&nbsp;&nbsp; @2 → right</code></div>
    </section>
  </div>

  <footer class="owner-footer"><span>Controller owns the shape</span><span>Renderer owns the syntax</span></footer>
</div>

<!--
The output is not an unexplained block of generated text. We can inspect the
semantic decisions beside the exact Rust they render to. The controller owns
the function shape; the renderer owns the punctuation and formatting.
-->

---
layout: full
---

<div class="slide-shell compose-slide">
  <header class="slide-header compact-header">
    <span class="section-number">03</span>
    <div>
      <p class="eyebrow">Composition, not autocomplete</p>
      <h2>The structure grows through small legal changes.</h2>
    </div>
  </header>

  <div class="progression">
    <div class="progress-step active"><span>01</span><b>Addition</b><small>function + add</small></div>
    <div class="progress-arrow">→</div>
    <div v-click class="progress-step active"><span>02</span><b>Multiplication</b><small>tuple + multiply</small></div>
    <div class="progress-arrow">→</div>
    <div v-click class="progress-step active"><span>03</span><b>Overflow safety</b><small>optional + checked</small></div>
  </div>

  <div class="composition-body">
    <section>
      <p class="eyebrow">Visible progression</p>
      <div class="meaning-line"><span>function</span><b>+</b><span>addition</span></div>
      <div v-click class="meaning-line"><span>tuple</span><b>+</b><span>multiplication</span></div>
      <div v-click class="meaning-line safe-line"><span>optional result</span><b>+</b><span>checked operations</span></div>
      <blockquote>Each request edits the representation before it becomes text.</blockquote>
    </section>

    <section class="code-window checked-code">
      <div class="code-title"><span>RUST</span><small>Final rendered output</small></div>
      <pre><code>fn calculate(left: i32, right: i32)
    -> Option&lt;(i32, i32)&gt; {
    let sum = left.checked_add(right)?;
    let product = left.checked_mul(right)?;
    Some((sum, product))
}</code></pre>
    </section>
  </div>
</div>

<!--
The next request does not regenerate the answer from scratch. Addition becomes
a tuple with multiplication, then checked operations make overflow explicit.
The visible progression is function plus addition, tuple plus multiplication,
optional result plus checked operations.
-->

---
layout: full
---

<div class="slide-shell controller-slide">
  <header class="slide-header compact-header">
    <span class="section-number">04</span>
    <div>
      <p class="eyebrow">The technical boundary</p>
      <h2>The best score only wins if the action is legal.</h2>
    </div>
  </header>

  <div class="controller-grid">
    <section class="score-table">
      <p class="table-label">Model scores</p>
      <div class="score-row"><code>ReturnString</code><span class="score-bar"><i style="width:91%"></i></span><b>0.91</b></div>
      <div class="score-row"><code>ReturnTuple</code><span class="score-bar"><i style="width:76%"></i></span><b>0.76</b></div>
      <div class="score-row"><code>ReturnInteger</code><span class="score-bar"><i style="width:63%"></i></span><b>0.63</b></div>
      <div class="score-row"><code>OpenBlock</code><span class="score-bar"><i style="width:28%"></i></span><b>0.28</b></div>
    </section>

    <section class="mask-column">
      <p class="table-label">Legality mask</p>
      <div class="mask-row illegal"><span>Masked</span><small>wrong result shape</small></div>
      <div class="mask-row legal"><span>Legal</span><small>fits current state</small></div>
      <div class="mask-row legal"><span>Legal</span><small>fits current state</small></div>
      <div class="mask-row illegal"><span>Masked</span><small>block unavailable</small></div>
    </section>

    <section v-click class="chosen-action">
      <p>Choose best legal</p>
      <strong>ReturnTuple</strong>
      <small>0.76</small>
    </section>
  </div>

  <div class="controller-loop">
    <span>Score</span><i>→</i><span>Mask</span><i>→</i><span>Choose</span><i>→</i><span>Apply</span><i>→</i><span class="render-step">Render</span>
  </div>
</div>

<!--
Underneath, the Rust prototype follows a small loop: score actions, mask
illegal actions, choose the best legal action, apply it, and render. A high
model score cannot override the controller's current state.
-->

---
layout: full
---

<div class="slide-shell collection-slide">
  <header class="slide-header compact-header">
    <span class="section-number">05</span>
    <div>
      <p class="eyebrow">Same mechanism, different pathway</p>
      <h2>Search a collection.</h2>
    </div>
  </header>

  <div class="collection-input">
    <span>Input</span>
    <div><b>7</b><b class="even">4</b><b class="even">2</b><b class="even">4</b><b>9</b><b class="even">6</b></div>
  </div>

  <div class="collection-route">
    <div class="route-spine"></div>
    <div class="route-row"><i></i><span class="semantic-tag">Find</span><strong>first even</strong><b>→</b><code>4</code><span class="semantic-tag green-tag">Option&lt;I32&gt;</span></div>
    <div v-click class="route-row"><i></i><span class="semantic-tag">Filter</span><strong>every even</strong><b>→</b><code>[4, 2, 4, 6]</code><span class="semantic-tag green-tag">Vec&lt;I32&gt;</span></div>
    <div v-click class="route-row"><i></i><span class="semantic-tag">Filter + Set</span><strong>unique even</strong><b>→</b><code>[4, 2, 6]</code><span class="semantic-tag green-tag">Vec&lt;I32&gt;</span></div>
  </div>

  <p class="bottom-statement">An authored route makes every click reproducible. The representation stays the same.</p>
</div>

<!--
The same mechanism handles searching, filtering, and stable de-duplication.
The web route is authored and deterministic for a reliable demo, but every
step still uses the same choose-meaning-then-render representation.
-->

---
layout: full
---

<div class="slide-shell scope-slide">
  <header class="slide-header compact-header">
    <span class="section-number">06</span>
    <div>
      <p class="eyebrow">What the prototype proves</p>
      <h2>Real, deterministic, and deliberately scoped.</h2>
    </div>
  </header>

  <div class="scope-table">
    <div class="scope-heading"><span>Demonstrated today</span><span>Next engineering step</span></div>
    <div class="scope-row"><span><i>✓</i> Interactive guided interface</span><span>Connect the UI to inference</span></div>
    <div class="scope-row"><span><i>✓</i> Inspectable semantic trace</span><span>Add tokenizer and training data</span></div>
    <div class="scope-row"><span><i>✓</i> Legality mask and controller</span><span>Extend scope-aware symbols</span></div>
    <div class="scope-row"><span><i>✓</i> Deterministic Rust renderer</span><span>Train and evaluate the model</span></div>
  </div>

  <div class="scope-note">
    <b>The current web UI and Rust model are independent prototypes.</b>
    <span>The UI uses fixed authored routes. The Rust model uses random weights and placeholder prompt IDs.</span>
  </div>
</div>

<!--
The web UI is real and interactive, but its routes are fixed and it does not
currently call the Rust model. The Rust model connects scoring, masking,
control, and rendering, but its weights are random. That is the honest boundary
of this prototype.
-->

---
layout: full
---

<div class="slide-shell final-slide">
  <div class="brand-lockup final-brand">
    <span class="brand-mark">S</span>
    <span>Shasha</span>
  </div>

  <p class="eyebrow">The reversal</p>
  <h2>The model is responsible<br>for <em>meaning.</em></h2>
  <h3>Correctness belongs to<br><strong>the system around it.</strong></h3>

  <div class="final-rule"></div>
  <p class="final-loop mono">SCORE → MASK → CHOOSE → APPLY → RENDER</p>
</div>

<!--
Shasha is not trying to make a language model responsible for every character
of a program. It makes the model responsible for meaning, and makes correctness
a property of the system around it. Stop here.
-->

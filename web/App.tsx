import { useState } from 'react'

type Action = { need: string; choice: string; meaning: string }
type Example = {
  id: string
  pathway: string
  step: number
  request: string
  title: string
  summary: string
  actions: Action[]
  symbols: string
  code: string[]
  next?: { label: string; description: string; id: string }
}
type Pathway = { id: string; number: string; title: string; description: string; start: string }

const pathways: Pathway[] = [
  { id: 'calculator', number: '01', title: 'Grow a calculator', description: 'Start with addition, introduce multiplication, then make both operations safe.', start: 'add' },
  { id: 'collections', number: '02', title: 'Search a collection', description: 'Find one even number, collect every match, then remove duplicates.', start: 'find-even' },
  { id: 'boundaries', number: '03', title: 'Handle boundaries', description: 'Clamp a value, validate its bounds, then group those bounds into a range.', start: 'clamp' },
]

const examples: Record<string, Example> = {
  add: {
    id: 'add', pathway: 'calculator', step: 1,
    request: 'Create a function that adds two integers',
    title: 'Begin with one typed operation',
    summary: 'Two integer parameters feed a single addition expression. The renderer owns the function syntax and punctuation.',
    actions: [
      { need: 'Item', choice: 'Function', meaning: 'Create one callable unit' },
      { need: 'Parameters', choice: 'I32 × 2', meaning: 'Accept two integer values' },
      { need: 'Return.type', choice: 'I32', meaning: 'Return one integer' },
      { need: 'Expression', choice: 'Add(@1, @2)', meaning: 'Add the two input symbols' },
    ],
    symbols: '@0 → calculate    @1 → left    @2 → right',
    code: ['fn calculate(left: i32, right: i32) -> i32 {', '    left + right', '}'],
    next: { label: 'Please now add multiplication', description: 'Keep the sum and return the product alongside it.', id: 'multiply' },
  },
  multiply: {
    id: 'multiply', pathway: 'calculator', step: 2,
    request: 'Please now add multiplication',
    title: 'Extend the structure with a second result',
    summary: 'The return type becomes a pair. Both expressions reuse the same scoped parameters.',
    actions: [
      { need: 'Return.type', choice: 'Tuple<I32, I32>', meaning: 'Return two integer results' },
      { need: 'Tuple.first', choice: 'Add(@1, @2)', meaning: 'Keep the original sum' },
      { need: 'Tuple.second', choice: 'Mul(@1, @2)', meaning: 'Add the requested product' },
    ],
    symbols: '@0 → calculate    @1 → left    @2 → right',
    code: ['fn calculate(left: i32, right: i32) -> (i32, i32) {', '    (left + right, left * right)', '}'],
    next: { label: 'Prevent integer overflow', description: 'Use checked operations and return nothing if either fails.', id: 'checked' },
  },
  checked: {
    id: 'checked', pathway: 'calculator', step: 3,
    request: 'Prevent integer overflow',
    title: 'Finish with checked arithmetic',
    summary: 'Both operations must succeed before the function constructs its result.',
    actions: [
      { need: 'Return.type', choice: 'Option<Tuple<I32, I32>>', meaning: 'Represent arithmetic failure' },
      { need: 'Bindings', choice: 'checked_add, checked_mul', meaning: 'Use safe integer operations' },
      { need: 'Expression', choice: 'Some(tuple)', meaning: 'Return values only when both exist' },
    ],
    symbols: '@0 → calculate    @1 → left    @2 → right',
    code: ['fn calculate(left: i32, right: i32) -> Option<(i32, i32)> {', '    let sum = left.checked_add(right)?;', '    let product = left.checked_mul(right)?;', '    Some((sum, product))', '}'],
  },
  'find-even': {
    id: 'find-even', pathway: 'collections', step: 1,
    request: 'Find the first even number in a list',
    title: 'Begin with a safe search',
    summary: 'The function borrows a slice and returns an optional value so a missing match is explicit.',
    actions: [
      { need: 'Param.type', choice: 'Ref<Slice<I32>>', meaning: 'Borrow a list of integers' },
      { need: 'Return.type', choice: 'Option<I32>', meaning: 'Allow for no match' },
      { need: 'Iterator', choice: 'Find', meaning: 'Stop at the first match' },
      { need: 'Predicate', choice: 'Eq(Mod(@2, 2), 0)', meaning: 'Recognise even values' },
    ],
    symbols: '@0 → first_even    @1 → numbers    @2 → number',
    code: ['fn first_even(numbers: &[i32]) -> Option<i32> {', '    numbers.iter().copied().find(|number| number % 2 == 0)', '}'],
    next: { label: 'Return every even number', description: 'Inspect the full input and collect all matches.', id: 'all-even' },
  },
  'all-even': {
    id: 'all-even', pathway: 'collections', step: 2,
    request: 'Return every even number',
    title: 'Expand the search into a filter',
    summary: 'The same predicate now keeps every match and collects the copied values.',
    actions: [
      { need: 'Return.type', choice: 'Vec<I32>', meaning: 'Return zero or more values' },
      { need: 'Iterator', choice: 'Filter', meaning: 'Inspect the complete input' },
      { need: 'Terminal', choice: 'Collect', meaning: 'Build the result vector' },
    ],
    symbols: '@0 → all_even    @1 → numbers    @2 → number',
    code: ['fn all_even(numbers: &[i32]) -> Vec<i32> {', '    numbers.iter().copied()', '        .filter(|number| number % 2 == 0)', '        .collect()', '}'],
    next: { label: 'Remove duplicate values', description: 'Preserve the first occurrence of each even number.', id: 'unique-even' },
  },
  'unique-even': {
    id: 'unique-even', pathway: 'collections', step: 3,
    request: 'Remove duplicate values',
    title: 'Finish with stable de-duplication',
    summary: 'A set tracks values already seen while the vector preserves input order.',
    actions: [
      { need: 'Local', choice: 'HashSet<I32>', meaning: 'Track values already emitted' },
      { need: 'Predicate', choice: 'Even && Inserted', meaning: 'Keep new even values only' },
      { need: 'Terminal', choice: 'Collect<Vec<I32>>', meaning: 'Preserve encounter order' },
    ],
    symbols: '@0 → unique_even    @1 → numbers    @2 → seen',
    code: ['fn unique_even(numbers: &[i32]) -> Vec<i32> {', '    let mut seen = std::collections::HashSet::new();', '    numbers.iter().copied()', '        .filter(|n| n % 2 == 0 && seen.insert(*n))', '        .collect()', '}'],
  },
  clamp: {
    id: 'clamp', pathway: 'boundaries', step: 1,
    request: 'Clamp a value between min and max',
    title: 'Begin with explicit boundaries',
    summary: 'The structure compares the value with each bound and returns the appropriate integer.',
    actions: [
      { need: 'Parameters', choice: 'I32 × 3', meaning: 'Receive a value and two bounds' },
      { need: 'Expression', choice: 'Clamp', meaning: 'Apply both boundaries' },
      { need: 'Lower.bound', choice: 'Max(@1, @2)', meaning: 'Raise values below minimum' },
      { need: 'Upper.bound', choice: 'Min(result, @3)', meaning: 'Lower values above maximum' },
    ],
    symbols: '@0 → clamp    @1 → value    @2 → min    @3 → max',
    code: ['fn clamp(value: i32, min: i32, max: i32) -> i32 {', '    value.max(min).min(max)', '}'],
    next: { label: 'Reject reversed bounds', description: 'Return an error when min exceeds max.', id: 'validate-clamp' },
  },
  'validate-clamp': {
    id: 'validate-clamp', pathway: 'boundaries', step: 2,
    request: 'Reject reversed bounds',
    title: 'Make invalid input explicit',
    summary: 'A result type separates a valid bounded value from an invalid range.',
    actions: [
      { need: 'Return.type', choice: 'Result<I32, Error>', meaning: 'Represent validation failure' },
      { need: 'Guard', choice: 'Gt(@2, @3)', meaning: 'Detect reversed bounds' },
      { need: 'Success', choice: 'Ok(Clamp)', meaning: 'Return a validated value' },
    ],
    symbols: '@0 → clamp    @1 → value    @2 → min    @3 → max',
    code: ['fn clamp(value: i32, min: i32, max: i32) -> Result<i32, &\'static str> {', '    if min > max { return Err("min must not exceed max"); }', '    Ok(value.max(min).min(max))', '}'],
    next: { label: 'Use an inclusive range', description: 'Keep the related bounds together in one value.', id: 'range-clamp' },
  },
  'range-clamp': {
    id: 'range-clamp', pathway: 'boundaries', step: 3,
    request: 'Use an inclusive range',
    title: 'Finish with one boundary value',
    summary: 'An inclusive range groups the two bounds in the function signature.',
    actions: [
      { need: 'Param.type', choice: 'RangeInclusive<I32>', meaning: 'Group minimum and maximum' },
      { need: 'Bindings', choice: 'Range.into_inner', meaning: 'Read both bounds once' },
      { need: 'Success', choice: 'Ok(Clamp)', meaning: 'Return the bounded value' },
    ],
    symbols: '@0 → clamp    @1 → value    @2 → range',
    code: ['fn clamp(value: i32, range: std::ops::RangeInclusive<i32>) -> Result<i32, &\'static str> {', '    let (min, max) = range.into_inner();', '    if min > max { return Err("range must not be empty"); }', '    Ok(value.max(min).min(max))', '}'],
  },
}

function CheckIcon() { return <svg viewBox="0 0 20 20" aria-hidden="true"><path d="m4 10 4 4 8-9" /></svg> }
function CopyIcon() { return <svg viewBox="0 0 20 20" aria-hidden="true"><rect x="7" y="7" width="9" height="9" rx="1" /><path d="M5 13H4a1 1 0 0 1-1-1V4a1 1 0 0 1 1-1h8a1 1 0 0 1 1 1v1" /></svg> }
function MenuIcon() { return <svg viewBox="0 0 20 20" aria-hidden="true"><path d="M4 6h12M4 10h12M4 14h12" /></svg> }
function ArrowIcon() { return <svg viewBox="0 0 20 20" aria-hidden="true"><path d="M4 10h11M11 6l4 4-4 4" /></svg> }
function PlusIcon() { return <svg viewBox="0 0 20 20" aria-hidden="true"><path d="M10 4v12M4 10h12" /></svg> }

export default function App() {
  const [currentId, setCurrentId] = useState<string | null>(null)
  const [history, setHistory] = useState<string[]>([])
  const [copiedId, setCopiedId] = useState<string | null>(null)
  const [sidebarOpen, setSidebarOpen] = useState(false)
  const current = currentId ? examples[currentId] : null
  const activePath = current ? pathways.find((path) => path.id === current.pathway) : null

  const start = (path: Pathway) => {
    setCurrentId(path.start); setHistory([path.start]); setCopiedId(null); setSidebarOpen(false)
    window.requestAnimationFrame(() => window.requestAnimationFrame(() => document.querySelector('.exchange:last-child')?.scrollIntoView({ behavior: 'smooth', block: 'start' })))
  }
  const continueTo = (id: string) => {
    setCurrentId(id); setHistory((items) => [...items, id]); setCopiedId(null)
    window.requestAnimationFrame(() => window.requestAnimationFrame(() => document.querySelector('.exchange:last-child')?.scrollIntoView({ behavior: 'smooth', block: 'start' })))
  }
  const reset = () => { setCurrentId(null); setHistory([]); setSidebarOpen(false) }
  const copyCode = async (example: Example) => {
    await navigator.clipboard.writeText(example.code.join('\n'))
    setCopiedId(example.id); window.setTimeout(() => setCopiedId(null), 1800)
  }

  return <div className="app-shell">
    <button className="mobile-menu" type="button" aria-label="Open pathway navigation" onClick={() => setSidebarOpen(true)}><MenuIcon /></button>
    <aside className={`sidebar ${sidebarOpen ? 'open' : ''}`}>
      <div className="sidebar-top">
        <button className="brand brand-button" type="button" aria-label="MoPaPPa home" onClick={reset}><span className="brand-mark">M</span><span>MoPaPPa</span></button>
        <button className="sidebar-close" type="button" aria-label="Close navigation" onClick={() => setSidebarOpen(false)}>×</button>
      </div>
      <button className="new-chat" type="button" onClick={reset}><PlusIcon /> New conversation</button>
      <div className="pathway-nav-heading"><span>Example pathways</span><small>Choose one</small></div>
      <nav className="pathway-nav" aria-label="Example pathways">
        {pathways.map((path) => <button type="button" key={path.id} onClick={() => start(path)} className={activePath?.id === path.id ? 'active' : ''}>
          <span>{path.number}</span><div><strong>{path.title}</strong><small>3 steps</small></div>
        </button>)}
      </nav>
      <div className="sidebar-note"><span>Deterministic by design</span><p>Every button follows an authored route. There is no free-text prompt or open-ended chat response.</p><a href="#method">Read the method <span aria-hidden="true">↗</span></a></div>
      <div className="profile"><span className="profile-mark">B</span><div><strong>Research workspace</strong><small>Constrained Rust IR</small></div><span aria-hidden="true">•••</span></div>
    </aside>
    {sidebarOpen && <button className="sidebar-scrim" aria-label="Close navigation" onClick={() => setSidebarOpen(false)} />}

    <main className="main-area">
      <header className="topbar"><div><strong>Structural interpreter</strong><span>Guided conversation</span></div><span className="route-count">3 fixed pathways</span></header>
      <div className="conversation" id="top">
        <div className="conversation-inner">
        {!current && <>
          <section className="explainer" id="method" aria-labelledby="explainer-title">
            <span className="eyebrow">Guided structural examples</span>
            <h1 id="explainer-title">The model chooses meaning.<br />The system handles syntax.</h1>
            <p className="explainer-copy">Choose an example below to begin. Each response offers a fixed next message, so the conversation stays predictable and reviewable.</p>
            <div className="method-line" aria-label="Example method">
              <div><span>01</span><strong>Prompt</strong><p>Choose one of three authored messages.</p></div>
              <div><span>02</span><strong>Inspect</strong><p>See the typed decisions behind the output.</p></div>
              <div><span>03</span><strong>Reply</strong><p>Continue with a suggested follow-up message.</p></div>
            </div>
          </section>
        </>}

        {history.length > 0 && <section className="thread" aria-label="Conversation">
          {history.map((id, messageIndex) => {
            const example = examples[id]
            return <div className="exchange" key={id}>
              <article className="message user-message">
                <div className="message-label">You</div>
                <div className="user-bubble">{example.request}</div>
              </article>
              <article className="message assistant-message">
                <div className="assistant-avatar" aria-hidden="true">M</div>
                <div className="assistant-content">
                  <div className="message-label">MoPaPPa · Step {messageIndex + 1} of 3</div>
                  <h2>{example.title}</h2>
                  <p className="response-summary">{example.summary}</p>
                  <div className="answer-grid">
                    <section className="trace" aria-labelledby={`trace-${id}`}>
                      <div className="section-heading"><span id={`trace-${id}`}>Structural decisions</span><small>{example.actions.length} decisions</small></div>
                      <div className="trace-list">{example.actions.map((action) => <div className="trace-row revealed" key={`${id}-${action.need}`}>
                        <span className="trace-check"><CheckIcon /></span><div><code>{action.need}</code><p>{action.meaning}</p></div><strong>{action.choice}</strong>
                      </div>)}</div>
                    </section>
                    <section className="code-panel" aria-labelledby={`code-${id}`}>
                      <div className="code-heading"><div><span>rust</span><small id={`code-${id}`}>Rendered output</small></div><button type="button" onClick={() => copyCode(example)}><CopyIcon /> {copiedId === id ? 'Copied' : 'Copy'}</button></div>
                      <pre>{example.code.map((line, index) => <span className="code-line" key={`${line}-${index}`}><i>{index + 1}</i><code>{line}</code></span>)}</pre>
                      <div className="code-meta"><span>Symbols</span><code>{example.symbols}</code></div>
                    </section>
                  </div>
                  <div className="validity-note"><CheckIcon /><p><strong>Valid by construction.</strong> Punctuation, braces, and indentation are supplied by the renderer.</p></div>
                </div>
              </article>
            </div>
          })}
        </section>}
        </div>
      </div>

      <div className="choice-tray-wrap" aria-label="Suggested messages">
        <div className="choice-tray">
          <div className="choice-tray-heading"><span>{current ? current.next ? 'Choose your next message' : 'Path complete' : 'Start with an example'}</span><small>{current ? activePath?.title : '3 pathways available'}</small></div>
          <div className="choice-options">
            {!current && pathways.map((path) => <button type="button" key={path.id} onClick={() => start(path)}><span>{examples[path.start].request}</span><ArrowIcon /></button>)}
            {current?.next && <button type="button" onClick={() => continueTo(current.next!.id)}><span>{current.next.label}</span><ArrowIcon /></button>}
            {current && !current.next && pathways.filter((path) => path.id !== current.pathway).map((path) => <button type="button" key={path.id} onClick={() => start(path)}><span>Try: {examples[path.start].request}</span><ArrowIcon /></button>)}
            {current && !current.next && <button type="button" className="secondary-choice" onClick={reset}><span>Start a new conversation</span><PlusIcon /></button>}
          </div>
        </div>
        <p>Choose a predefined message to continue this deterministic demo.</p>
      </div>
    </main>
  </div>
}

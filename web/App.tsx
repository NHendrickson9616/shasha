import { FormEvent, useEffect, useMemo, useState } from 'react'

type Action = {
  need: string
  choice: string
  meaning: string
  rejected: string
}

type Scenario = {
  match: RegExp
  request: string
  title: string
  summary: string
  actions: Action[]
  symbols: string
  code: string[]
}

type AppMemory = {
  currentRequest: string
  recentRequests: string[]
}

const MEMORY_KEY = 'mopappa-memory-v1'

const emptyMemory: AppMemory = {
  currentRequest: '',
  recentRequests: [],
}

function loadMemory(): AppMemory {
  try {
    const stored = window.localStorage.getItem(MEMORY_KEY)
    if (!stored) return emptyMemory

    const parsed = JSON.parse(stored) as Partial<AppMemory>
    const currentRequest = typeof parsed.currentRequest === 'string' ? parsed.currentRequest : ''
    const recentRequests = Array.isArray(parsed.recentRequests)
      ? parsed.recentRequests.filter((item): item is string => typeof item === 'string').slice(0, 5)
      : []

    return { currentRequest, recentRequests }
  } catch {
    return emptyMemory
  }
}

const scenarios: Scenario[] = [
  {
    match: /double|twice|multiply/i,
    request: 'Double an integer',
    title: 'A typed function with one multiplication',
    summary: 'The structure needs one integer parameter, an integer return type, and a multiplication expression that reuses the input symbol.',
    actions: [
      { need: 'Item', choice: 'Function', meaning: 'Create one callable unit', rejected: 'If · Call · I32' },
      { need: 'Param.binding', choice: 'NewSymbol @1', meaning: 'Accept one input value', rejected: 'SymbolRef · Literal' },
      { need: 'Param.type', choice: 'I32', meaning: 'The input is an integer', rejected: 'Function · BinaryOp' },
      { need: 'Return.type', choice: 'I32', meaning: 'The result is an integer', rejected: 'Call · Mul · If' },
      { need: 'Expression', choice: 'BinaryOp', meaning: 'Compute from two operands', rejected: 'Function · I32' },
      { need: 'Binary.operator', choice: 'Mul', meaning: 'Scale the input value', rejected: 'If · Param · Call' },
      { need: 'Binary.left', choice: 'SymbolRef @1', meaning: 'Reuse the function input', rejected: 'Function · Type' },
      { need: 'Binary.right', choice: 'IntLiteral 2', meaning: 'Multiply it by two', rejected: 'Param · Item · Type' },
    ],
    symbols: '@0 → double    @1 → value',
    code: ['fn double(value: i32) -> i32 {', '    value * 2', '}'],
  },
  {
    match: /even|list|array|vector|find/i,
    request: 'Find the first even number in a list',
    title: 'A safe search with an optional result',
    summary: 'The structure borrows the list, checks each value once, and returns an Option so an empty result is represented explicitly.',
    actions: [
      { need: 'Item', choice: 'Function', meaning: 'Create a reusable search', rejected: 'If · Loop · I32' },
      { need: 'Param.type', choice: 'Ref<Slice<I32>>', meaning: 'Read a borrowed list', rejected: 'Function · Call' },
      { need: 'Return.type', choice: 'Option<I32>', meaning: 'The search may find nothing', rejected: 'BinaryOp · Loop' },
      { need: 'Block.entry', choice: 'ForLoop', meaning: 'Inspect each number once', rejected: 'Type · Param' },
      { need: 'Loop.condition', choice: 'Eq(Mod(@2, 2), 0)', meaning: 'Recognise an even value', rejected: 'Function · Struct' },
      { need: 'Then.expression', choice: 'Return(Some(@2))', meaning: 'Finish at the first match', rejected: 'Type · Param' },
      { need: 'Block.tail', choice: 'None', meaning: 'Represent no match found', rejected: 'Item · Operator' },
    ],
    symbols: '@0 → first_even    @1 → numbers    @2 → number',
    code: [
      'fn first_even(numbers: &[i32]) -> Option<i32> {',
      '    for &number in numbers {',
      '        if number % 2 == 0 {',
      '            return Some(number);',
      '        }',
      '    }',
      '    None',
      '}',
    ],
  },
  {
    match: /clamp|minimum|maximum|between|range/i,
    request: 'Clamp a value between min and max',
    title: 'A bounded value with explicit branches',
    summary: 'The structure compares the value with each boundary in turn, returning the appropriate bound or the original value.',
    actions: [
      { need: 'Item', choice: 'Function', meaning: 'Create a reusable constraint', rejected: 'If · Call · I32' },
      { need: 'Params', choice: 'I32 × 3', meaning: 'Receive value and two bounds', rejected: 'Expression · Item' },
      { need: 'Return.type', choice: 'I32', meaning: 'Return a bounded integer', rejected: 'If · Function' },
      { need: 'Expression', choice: 'If', meaning: 'Branch on boundary checks', rejected: 'Type · Param' },
      { need: 'If.condition', choice: 'Lt(@1, @2)', meaning: 'Detect values below minimum', rejected: 'Item · Type' },
      { need: 'Else.expression', choice: 'If', meaning: 'Check the upper bound next', rejected: 'Param · Function' },
      { need: 'If.condition', choice: 'Gt(@1, @3)', meaning: 'Detect values above maximum', rejected: 'Item · Type' },
      { need: 'Else.tail', choice: 'SymbolRef @1', meaning: 'Keep values already in range', rejected: 'Function · Param' },
    ],
    symbols: '@0 → clamp    @1 → value    @2 → min    @3 → max',
    code: [
      'fn clamp(value: i32, min: i32, max: i32) -> i32 {',
      '    if value < min {',
      '        min',
      '    } else if value > max {',
      '        max',
      '    } else {',
      '        value',
      '    }',
      '}',
    ],
  },
]

function PlusIcon() {
  return <svg viewBox="0 0 20 20" aria-hidden="true"><path d="M10 4v12M4 10h12" /></svg>
}

function SendIcon() {
  return <svg viewBox="0 0 20 20" aria-hidden="true"><path d="m4 10 6-6 6 6M10 4v12" /></svg>
}

function CheckIcon() {
  return <svg viewBox="0 0 20 20" aria-hidden="true"><path d="m4 10 4 4 8-9" /></svg>
}

function CopyIcon() {
  return <svg viewBox="0 0 20 20" aria-hidden="true"><rect x="7" y="7" width="9" height="9" rx="1" /><path d="M5 13H4a1 1 0 0 1-1-1V4a1 1 0 0 1 1-1h8a1 1 0 0 1 1 1v1" /></svg>
}

function MenuIcon() {
  return <svg viewBox="0 0 20 20" aria-hidden="true"><path d="M4 6h12M4 10h12M4 14h12" /></svg>
}

function ResetIcon() {
  return <svg viewBox="0 0 20 20" aria-hidden="true"><path d="M4 6h12M8 3h4l1 3M6 6l1 11h6l1-11M9 9v5M11 9v5" /></svg>
}

export default function App() {
  const [initialMemory] = useState(loadMemory)
  const restoredScenario = initialMemory.currentRequest
    ? scenarios.find((item) => item.match.test(initialMemory.currentRequest)) ?? scenarios[0]
    : null
  const [input, setInput] = useState('')
  const [request, setRequest] = useState(initialMemory.currentRequest)
  const [scenario, setScenario] = useState<Scenario | null>(restoredScenario)
  const [visibleSteps, setVisibleSteps] = useState(restoredScenario?.actions.length ?? 0)
  const [isRunning, setIsRunning] = useState(false)
  const [copied, setCopied] = useState(false)
  const [sidebarOpen, setSidebarOpen] = useState(false)
  const [recentRequests, setRecentRequests] = useState<string[]>(initialMemory.recentRequests)

  useEffect(() => {
    if (!request && recentRequests.length === 0) {
      window.localStorage.removeItem(MEMORY_KEY)
      return
    }

    window.localStorage.setItem(MEMORY_KEY, JSON.stringify({
      currentRequest: request,
      recentRequests,
    } satisfies AppMemory))
  }, [recentRequests, request])

  useEffect(() => {
    if (!isRunning || !scenario) return
    if (visibleSteps >= scenario.actions.length) {
      const finish = window.setTimeout(() => setIsRunning(false), 260)
      return () => window.clearTimeout(finish)
    }
    const next = window.setTimeout(() => setVisibleSteps((count) => count + 1), 180)
    return () => window.clearTimeout(next)
  }, [isRunning, scenario, visibleSteps])

  const progress = useMemo(
    () => scenario ? Math.round((visibleSteps / scenario.actions.length) * 100) : 0,
    [scenario, visibleSteps],
  )

  const run = (raw: string) => {
    const request = raw.trim()
    if (!request) return
    const nextScenario = scenarios.find((item) => item.match.test(request)) ?? scenarios[0]
    setRequest(request)
    setScenario(nextScenario)
    setVisibleSteps(0)
    setCopied(false)
    setIsRunning(true)
    setInput('')
    setSidebarOpen(false)
    setRecentRequests((current) => [request, ...current.filter((item) => item !== request)].slice(0, 5))
    window.requestAnimationFrame(() => {
      document.querySelector('.conversation')?.scrollTo({ top: 9999, behavior: 'smooth' })
    })
  }

  const submit = (event: FormEvent) => {
    event.preventDefault()
    run(input)
  }

  const copyCode = async () => {
    if (!scenario) return
    await navigator.clipboard.writeText(scenario.code.join('\n'))
    setCopied(true)
    window.setTimeout(() => setCopied(false), 1800)
  }

  const startNewConversation = () => {
    setRequest('')
    setScenario(null)
    setVisibleSteps(0)
    setIsRunning(false)
    setCopied(false)
    setInput('')
    setSidebarOpen(false)
  }

  const resetMemory = () => {
    window.localStorage.removeItem(MEMORY_KEY)
    setRecentRequests([])
    startNewConversation()
  }

  return (
    <div className="app-shell">
      <button className="mobile-menu" type="button" aria-label="Open navigation" onClick={() => setSidebarOpen(true)}><MenuIcon /></button>

      <aside className={`sidebar ${sidebarOpen ? 'open' : ''}`}>
        <div className="sidebar-top">
          <a className="brand" href="#top" aria-label="MoPaPPa home"><span className="brand-mark">M</span><span>MoPaPPa</span></a>
          <button className="sidebar-close" type="button" aria-label="Close navigation" onClick={() => setSidebarOpen(false)}>×</button>
        </div>

        <button className="new-chat" type="button" onClick={startNewConversation}><PlusIcon /> New conversation</button>

        <nav className="conversation-nav" aria-label="Recent conversations">
          <div className="conversation-nav-heading"><span>Recent</span><small>Saved locally</small></div>
          {recentRequests.length === 0 && <p className="no-recents">No conversations yet</p>}
          {recentRequests.map((item) => (
            <button
              type="button"
              key={item}
              onClick={() => run(item)}
              className={request === item ? 'active' : ''}
            >
              {item}
            </button>
          ))}
        </nav>

        <button className="reset-memory" type="button" onClick={resetMemory} disabled={recentRequests.length === 0 && !request}>
          <ResetIcon /> Reset memory
        </button>

        <div className="sidebar-note">
          <span>What is different?</span>
          <p>The model predicts program structure. A deterministic renderer supplies valid Rust syntax.</p>
          <a href="#method">Read the method <span aria-hidden="true">↗</span></a>
        </div>

        <div className="profile">
          <span className="profile-mark">B</span>
          <div><strong>Research workspace</strong><small>Constrained Rust IR</small></div>
          <span aria-hidden="true">•••</span>
        </div>
      </aside>

      {sidebarOpen && <button className="sidebar-scrim" aria-label="Close navigation" onClick={() => setSidebarOpen(false)} />}

      <main className="main-area">
        <header className="topbar">
          <div><strong>Structural interpreter</strong><span>MoPaPPa research preview</span></div>
          <button type="button" className="mode-button">Rust <span aria-hidden="true">⌄</span></button>
        </header>

        <div className="conversation" id="top">
          <div className="conversation-inner">
            <section className="explainer" id="method" aria-labelledby="explainer-title">
              <span className="eyebrow">Structure-aware code generation</span>
              <h1 id="explainer-title">The model chooses meaning.<br />The system handles syntax.</h1>
              <p className="explainer-copy">Describe a small program in plain language. MoPaPPa builds a typed structure, rules out invalid moves, and renders legal Rust only after the structure is complete.</p>

              <div className="method-line" aria-label="Generation method">
                <div><span>01</span><strong>Interpret</strong><p>Convert your intent into program concepts.</p></div>
                <div><span>02</span><strong>Constrain</strong><p>Offer only valid choices at each position.</p></div>
                <div><span>03</span><strong>Render</strong><p>Add grammar and formatting deterministically.</p></div>
              </div>

              <div className="suggestions" aria-label="Example prompts">
                <span>Try an example</span>
                {scenarios.map((item) => (
                  <button type="button" key={item.request} onClick={() => run(item.request)} disabled={isRunning}>{item.request}</button>
                ))}
              </div>
            </section>

            {scenario && <section className="thread" aria-label="Conversation">
              <article className="message user-message">
                <div className="message-label">You</div>
                <div className="user-bubble">{request}</div>
              </article>

              <article className="message assistant-message">
                <div className="assistant-avatar" aria-hidden="true">M</div>
                <div className="assistant-content">
                  <div className="message-label">MoPaPPa</div>
                  <h2>{scenario.title}</h2>
                  <p className="response-summary">{scenario.summary}</p>

                  <div className="generation-status" aria-live="polite">
                    <div><span>{isRunning ? 'Building structure' : 'Structure complete'}</span><strong>{progress}%</strong></div>
                    <div className="progress-track"><i style={{ width: `${progress}%` }} /></div>
                    <p>{isRunning ? `Selecting ${scenario.actions[Math.min(visibleSteps, scenario.actions.length - 1)].need}` : 'Every selected node satisfies the current schema and scope.'}</p>
                  </div>

                  <div className="answer-grid">
                    <section className="trace" aria-labelledby="trace-title">
                      <div className="section-heading"><span id="trace-title">Structural decisions</span><small>{scenario.actions.length} decisions</small></div>
                      <div className="trace-list">
                        {scenario.actions.map((action, index) => {
                          const revealed = index < visibleSteps
                          const active = isRunning && index === visibleSteps
                          return (
                            <div className={`trace-row ${revealed ? 'revealed' : ''} ${active ? 'active' : ''}`} key={`${scenario.request}-${action.need}`}>
                              <span className="trace-check">{revealed ? <CheckIcon /> : index + 1}</span>
                              <div><code>{action.need}</code><p>{revealed ? action.meaning : active ? 'Evaluating valid choices…' : 'Waiting for prior decision'}</p></div>
                              <strong>{revealed ? action.choice : active ? 'scoring…' : '—'}</strong>
                            </div>
                          )
                        })}
                      </div>
                    </section>

                    <section className="code-panel" aria-labelledby="code-title">
                      <div className="code-heading">
                        <div><span>rust</span><small id="code-title">Rendered output</small></div>
                        <button type="button" onClick={copyCode} disabled={isRunning}><CopyIcon /> {copied ? 'Copied' : 'Copy'}</button>
                      </div>
                      <pre className={isRunning ? 'muted-code' : ''}>{scenario.code.map((line, index) => (
                        <span className="code-line" key={`${line}-${index}`}><i>{index + 1}</i><code>{line}</code></span>
                      ))}</pre>
                      <div className="code-meta"><span>Symbols</span><code>{scenario.symbols}</code></div>
                    </section>
                  </div>

                  <div className="validity-note"><CheckIcon /><p><strong>Valid by construction.</strong> Punctuation, braces, and indentation are supplied by the renderer rather than predicted by the model.</p></div>
                </div>
              </article>
            </section>}
          </div>
        </div>

        <div className="composer-wrap">
          <form className="composer" onSubmit={submit}>
            <textarea
              aria-label="Message MoPaPPa"
              value={input}
              onChange={(event) => setInput(event.target.value)}
              onKeyDown={(event) => {
                if (event.key === 'Enter' && !event.shiftKey) {
                  event.preventDefault()
                  if (input.trim() && !isRunning) run(input)
                }
              }}
              placeholder="Describe the Rust program you want to build"
              rows={1}
            />
            <div className="composer-actions">
              <span><b>+</b> Plain language is enough</span>
              <button type="submit" disabled={!input.trim() || isRunning} aria-label="Send message"><SendIcon /></button>
            </div>
          </form>
          <p className="composer-note">MoPaPPa is a structural generation demo. Review generated code before use.</p>
        </div>
      </main>
    </div>
  )
}

import { FormEvent, useEffect, useState } from 'react'

type Step = { action: string }
type Scenario = { match: RegExp; code: string; steps: Step[] }

const scenarios: Scenario[] = [
  {
    match: /even|list|array|vector|find/i,
    code: `fn first_even(numbers: &[i32]) -> Option<i32> {
    for &number in numbers {
        if number % 2 == 0 {
            return Some(number);
        }
    }
    None
}`,
    steps: [{ action: 'Create a function' }, { action: 'Read a list of numbers' }, { action: 'Check each number' }, { action: 'Return the first even number' }],
  },
  {
    match: /clamp|minimum|maximum|between|range/i,
    code: `fn clamp(value: i32, min: i32, max: i32) -> i32 {
    if value < min {
        min
    } else if value > max {
        max
    } else {
        value
    }
}`,
    steps: [{ action: 'Create a function' }, { action: 'Compare the value with the minimum' }, { action: 'Compare the value with the maximum' }, { action: 'Return the value inside the range' }],
  },
  {
    match: /.*/,
    code: `fn double(value: i32) -> i32 {
    value * 2
}`,
    steps: [{ action: 'Create a function' }, { action: 'Read one integer' }, { action: 'Multiply it by two' }, { action: 'Return the result' }],
  },
]

const examples = ['Double an integer', 'Find the first even number in a list', 'Clamp a value between min and max']

function Arrow() {
  return <svg viewBox="0 0 20 20" aria-hidden="true"><path d="M4 10h12M11 5l5 5-5 5" /></svg>
}

export default function App() {
  const [input, setInput] = useState('')
  const [request, setRequest] = useState('')
  const [result, setResult] = useState<Scenario | null>(null)
  const [isBuilding, setIsBuilding] = useState(false)

  useEffect(() => {
    if (!isBuilding) return
    const timer = window.setTimeout(() => setIsBuilding(false), 650)
    return () => window.clearTimeout(timer)
  }, [isBuilding, request])

  const run = (raw: string) => {
    const text = raw.trim()
    if (!text) return
    setRequest(text)
    setResult(scenarios.find((scenario) => scenario.match.test(text)) ?? scenarios[2])
    setIsBuilding(true)
    setInput('')
  }

  const submit = (event: FormEvent) => {
    event.preventDefault()
    run(input)
  }

  return (
    <div className="app">
      <header>
        <a className="brand" href="#top" aria-label="MoPaPPa home">MoPaPPa</a>
        <span>Rust code demo</span>
      </header>

      <main id="top">
        <section className="intro">
          <p className="eyebrow">STRUCTURE FIRST. CODE SECOND.</p>
          <h1>Describe a small Rust function.</h1>
          <p>MoPaPPa turns your request into program structure, then writes the Rust syntax for you.</p>

          <form onSubmit={submit}>
            <label htmlFor="prompt">What should it build?</label>
            <div className="input-row">
              <input id="prompt" value={input} onChange={(event) => setInput(event.target.value)} placeholder="For example: double an integer" autoComplete="off" />
              <button type="submit" disabled={!input.trim() || isBuilding}>Build <Arrow /></button>
            </div>
          </form>

          <div className="examples" aria-label="Example requests">
            <span>Try:</span>
            {examples.map((example) => <button key={example} onClick={() => run(example)} disabled={isBuilding}>{example}</button>)}
          </div>
        </section>

        <section className="output" aria-live="polite">
          {!result && <div className="empty"><span>01</span><h2>Your Rust code will appear here.</h2><p>Choose an example or write a request above.</p></div>}
          {result && isBuilding && <div className="building"><span className="loader" /><div><h2>Building the structure…</h2><p>{request}</p></div></div>}
          {result && !isBuilding && (
            <div className="result">
              <div className="result-heading"><div><span>RESULT</span><h2>{request}</h2></div><b>Ready</b></div>
              <pre><code>{result.code}</code></pre>
              <details><summary>How it was built</summary><ol>{result.steps.map((step) => <li key={step.action}>{step.action}</li>)}</ol></details>
            </div>
          )}
        </section>
      </main>

      <footer>Illustrative demo · Supports three example requests</footer>
    </div>
  )
}

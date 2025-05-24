import { useEffect, useState } from 'react'
import init, { parse } from 'block-parser-wasm'

export default function App() {
  const [output, setOutput] = useState<any[]>([])
  const [ready, setReady] = useState(false)
  const [text, setText] = useState('hello world')

  useEffect(() => {
    init().then(() => setReady(true))
  }, [])

  useEffect(() => {
    if (!ready) return

    const result = parse(text)
    setOutput(result)
  }, [ready, text])

  return (
    <div>
      <h1>Parsed Output</h1>
      <pre>{JSON.stringify(output, null, 2)}</pre>
    </div>
  )
}

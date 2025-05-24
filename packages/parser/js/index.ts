type Block = {
  blockName: string | null
  attrs: Record<string, any>
  innerBlocks: any[]
  innerHTML: string
  innerContent: (string | null)[]
}

let wasm: any = null
let initialized = false

export default async function init(): Promise<void> {
  if (initialized) return
  wasm = await import('block-parser-wasm.js')
  await wasm.default()
  initialized = true
}

export function parse(input: string): Block[] {
  if (!wasm) throw new Error('WASM not initialized. Call `init()` first.')
  return wasm.parse_blocks(input)
}

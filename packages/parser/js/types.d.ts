declare module 'block-parser-wasm.js' {
  const init: () => Promise<void>
  export default init
  export function parse_blocks(input: string): any
}

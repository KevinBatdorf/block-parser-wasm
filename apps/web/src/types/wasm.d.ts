declare module '*.wasm?url' {
  const url: string;
  export default url;
}
declare module 'block-parser-wasm' {
  const init: (moduleOrPath?: RequestInfo | URL) => Promise<void>;
  export default init;
  export function parse(input: string): any;
}

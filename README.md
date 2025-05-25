A 10x slower mostly-spec-compatible WebAssembly parser for WordPress blocks.

You can try it out by running `npm ci && npm run dev` and then opening `http://localhost:4200`. In apps/web/src/app.tsx you can edit the code snippet to see the output.

### TODO
- Make it faster
- Maybe a webui to copy/paste blocks

### Notes
- I used the spec from [block-serialization-spec-parser](https://github.com/WordPress/gutenberg/tree/trunk/packages/block-serialization-spec-parser)
- I implemented all the tests from that repo [here](https://github.com/KevinBatdorf/block-parser-wasm/blob/main/packages/parser/tests/parser.rs)
- It diverges from the spec in the way it handles malformed blocks. I think this is a better implementation as it will attempt to parse the block as if the closing tag exists.

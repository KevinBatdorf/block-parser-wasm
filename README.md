A slower mostly-spec-compatible WebAssembly parser for WordPress blocks. The test block in `apps/web/src/assets/block.ts` takes about 4ms to parse, meanwhile in the core parser it takes about 0.4ms.

You can try it out by running `npm ci && npm run dev` and then opening `http://localhost:4200`. In apps/web/src/app.tsx you can edit the code snippet to see the output.

### TODO
- Rewrite as a recursive descent parser

### Notes
- I used the spec from [block-serialization-spec-parser](https://github.com/WordPress/gutenberg/tree/trunk/packages/block-serialization-spec-parser)
- I implemented all the tests from that repo [here](https://github.com/KevinBatdorf/block-parser-wasm/blob/main/packages/parser/tests/parser.rs)
- It diverges from the spec in the way it handles malformed blocks. I think this is a better implementation as it will attempt to parse the block as if the closing tag exists.
- It also diverges by stripping out a null block if you have whitespace at the start/end of a block. This seems more useful to me as well since the user almost certainly doesn't want a null block with just whitespace (e.g. a newline char).
_ It also will include `isFreeform: true` on freeform blocks, which is not also in the spec.

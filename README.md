A slower mostly-spec-compatible (see notes below) WebAssembly parser for WordPress blocks. The test block in `apps/web/src/blocks/block.ts` takes about 2ms using the rust binary, and 4ms to parse via webassembly. Meanwhile, the default parser in WP it takes about 0.4ms.

You can try it out by running `npm ci && npm run dev` and then opening `http://localhost:4200`. In apps/web/src/app.tsx you can edit the code snippet to see the output.

### TODO
- Implement using nom or a handmade parser

### Notes
- I used the spec from [block-serialization-spec-parser](https://github.com/WordPress/gutenberg/tree/trunk/packages/block-serialization-spec-parser)
- I implemented all the tests from that repo [here](https://github.com/KevinBatdorf/block-parser-wasm/blob/main/packages/parser/tests/parser.rs)
- It diverges from the spec in the way it handles malformed blocks. I think this is a better implementation as it will attempt to parse the block as if the closing tag exists.
- It also diverges by stripping out a null block if you have whitespace at the start/end of a block. This seems more useful to me as well since the user almost certainly doesn't want a null block with just whitespace (e.g. a newline char).
- It will also include `core/freeform` on freeform blocks blockName field, which too is not in the spec.

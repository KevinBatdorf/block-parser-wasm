A 10x slower mostly-spec-compatible WebAssembly parser for WordPress blocks.

You can try it out by running `npm ci && npm run dev` and then opening `http://localhost:4200`. In apps/web/src/app.tsx you can edit the code snippet to see the output.

### TODO
- Make it faster
- Maybe a webui to copy/paste blocks

### Notes
- It diverges from the spec in the way it handles malformed blocks. I think this is a better implementation as it will attempt to parse the block as if the closing tag is in the right place.

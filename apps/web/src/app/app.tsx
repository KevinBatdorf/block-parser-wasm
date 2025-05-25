import { useEffect, useState } from "react";
import init, { parse } from "block-parser-wasm";
import { parse as wpParse } from "@wordpress/block-serialization-default-parser";
import { deepEqual } from "fast-equals";

export default function App() {
	// biome-ignore lint/suspicious/noExplicitAny: todo
	const [output, setOutput] = useState<any[]>([]);
	// biome-ignore lint/suspicious/noExplicitAny: todo
	const [wpOutput, setWpOutput] = useState<any[]>([]);
	const [areSame, setAreSame] = useState(false);
	const [ready, setReady] = useState(false);
	const [text] = useState(`<!-- wp:block -->content<!-- /wrong:block -->`);

	useEffect(() => {
		init().then(() => setReady(true));
	}, []);

	useEffect(() => {
		if (!ready) return;
		const mine = parse(text);
		setOutput(mine);
		const wp = wpParse(text);
		setWpOutput(wp);
		setAreSame(deepEqual(mine, wp));
	}, [ready, text]);

	return (
		<div className="flex flex-col items-center mx-auto p-4">
			{areSame ? (
				<link
					rel="icon"
					href="data:image/svg+xml,<svg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 100 100'><text y='.9em' font-size='90'>✅</text></svg>"
				/>
			) : (
				<link
					rel="icon"
					href="data:image/svg+xml,<svg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 100 100'><text y='.9em' font-size='90'>❌</text></svg>"
				/>
			)}
			<h1 className="text-2xl font-bold mb-4">Block Parser Comparison</h1>
			<div className="flex">
				<div className="">
					<h1>Parsed Output</h1>
					<pre className="text-pretty">{JSON.stringify(output, null, 2)}</pre>
				</div>
				<div className="">
					<h1>WordPress Parser Output</h1>
					<pre className="text-pretty">{JSON.stringify(wpOutput, null, 2)}</pre>
				</div>
			</div>
		</div>
	);
}

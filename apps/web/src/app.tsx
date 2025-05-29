import { useEffect, useState } from "react";
import init, { parse } from "block-parser-wasm";
import { parse as wpParse } from "@wordpress/block-serialization-default-parser";
import { deepEqual } from "fast-equals";
import defaultTest from "./assets/block";

const ITERATIONS = 1_000;

export default function App() {
	const [output, setOutput] = useState<any[]>([]);
	const [wpOutput, setWpOutput] = useState<any[]>([]);
	const [areSame, setAreSame] = useState(false);
	const [ready, setReady] = useState(false);
	const [timing, setTiming] = useState<{ wasm: number; wp: number }>({
		wasm: 0,
		wp: 0,
	});

	const [text] = useState(defaultTest);

	useEffect(() => {
		new Promise((resolve) => setTimeout(resolve, 1000)).then(() => {
			init().then(() => setReady(true));
		});
	}, []);

	useEffect(() => {
		if (!ready || !text) return;

		setTimeout(() => {
			// Warm-up phase
			for (let i = 0; i < 1000; i++) {
				parse(text);
				wpParse(text);
			}

			let wasmOut = null;
			let wpOut = null;

			// Timed WASM
			const t0 = performance.now();
			for (let i = 0; i < ITERATIONS; i++) {
				wasmOut = parse(text);
			}
			const t1 = performance.now();
			// wasmOut = parse(text);

			// Timed WP
			const t2 = performance.now();
			for (let i = 0; i < ITERATIONS; i++) {
				wpOut = wpParse(text);
			}
			const t3 = performance.now();
			// wpOut = wpParse(text);

			setOutput(wasmOut);
			setWpOutput(wpOut ?? []);
			setAreSame(deepEqual(wasmOut, wpOut));
			setTiming({
				wasm: (t1 - t0) / ITERATIONS,
				wp: (t3 - t2) / ITERATIONS,
			});
		}, 0); // yields to event loop
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

			<h1 className="text-2xl font-bold mb-2">Block Parser Benchmark</h1>
			{timing.wasm ? (
				<>
					<p>Ran each parser {ITERATIONS.toLocaleString()} times</p>
					<p className="mb-4">
						WASM avg: {timing.wasm.toFixed(4)}ms | WP avg:{" "}
						{timing.wp.toFixed(4)}ms
					</p>
				</>
			) : (
				"Doing stuff..."
			)}

			<div className="flex gap-4 w-full max-w-5xl">
				<div className="w-1/2">
					<h2 className="font-bold text-lg">Your WASM Parser</h2>
					<pre className="text-sm overflow-x-auto max-h-[400px]">
						{JSON.stringify(output, null, 2)}
					</pre>
				</div>
				<div className="w-1/2">
					<h2 className="font-bold text-lg">WordPress JS Parser</h2>
					<pre className="text-sm overflow-x-auto max-h-[400px]">
						{JSON.stringify(wpOutput, null, 2)}
					</pre>
				</div>
			</div>
		</div>
	);
}

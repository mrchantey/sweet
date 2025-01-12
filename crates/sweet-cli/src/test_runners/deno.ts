// @ts-nocheck
// deno-lint-ignore-file 
// wasm-bindgen deno runner https://vscode.dev/github/rustwasm/wasm-bindgen/blob/main/crates/cli/src/bin/wasm-bindgen-test-runner/deno.rs
import init from './bindgen.js'

// THIS FILE IS CACHED
// ⚠️⚠️⚠️⚠️ For the love of god delete target/sweet/deno.ts in between changes ⚠️⚠️⚠️⚠️

// cwd will be relative to the crate, ie
// cargo test --workspace is different cwd from cargo test -p my_crate
// so we try to use absolute dir

// wrapper function to avoid abort
globalThis.panic_to_error = f => f();
globalThis.cwd = () => Deno.cwd()
globalThis.read_file = (path: string) => {
	try {
		return Deno.readTextFileSync(path)
	} catch (err) {
		return null
	}
};

const wasm = await init()
	.catch((_err: any) => {
		// sync tests failed
		Deno.exit(1);
	})

// may not exist and thats ok
await wasm.run_with_pending?.().catch((_err: any) => {
	// async tests failed
	Deno.exit(1);
})
// @ts-nocheck
// deno-lint-ignore-file 
// wasm-bindgen deno runner https://vscode.dev/github/rustwasm/wasm-bindgen/blob/main/crates/cli/src/bin/wasm-bindgen-test-runner/deno.rs
import init from './bindgen.js'

// THIS FILE IS CACHED
// During development, delete target/sweet/deno.ts in between changes

// wrapper function to avoid abort
globalThis.__wbg_test_invoke = f => f();
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
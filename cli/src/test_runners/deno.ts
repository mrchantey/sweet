// @ts-nocheck
// deno-lint-ignore-file 
//
// wasm-bindgen deno runner https://vscode.dev/github/rustwasm/wasm-bindgen/blob/main/crates/cli/src/bin/wasm-bindgen-test-runner/deno.rs
//

import init from './bindgen.js'

// wrapper function to avoid abort
globalThis.__wbg_test_invoke = f => f();
globalThis.read_file = (path:string) => Deno.readTextFileSync(path);

const wasm = await init()
	.catch((_err: any) => {
		// sync tests failed
		Deno.exit(1);
	})

await wasm.run_with_pending().catch((_err: any) => {
	// async tests failed
	Deno.exit(1);
})
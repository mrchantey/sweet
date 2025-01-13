// @ts-nocheck
// deno-lint-ignore-file 
// wasm-bindgen deno runner https://vscode.dev/github/rustwasm/wasm-bindgen/blob/main/crates/cli/src/bin/wasm-bindgen-test-runner/deno.rs
import init from './bindgen.js'


/** 
This file is cached and will be replaced on hash change
cwd will be relative to the crate, ie
cargo test --workspace is different cwd from cargo test -p my_crate
so we try to use absolute dir


implentations of js_runtime
see [js_runtime_extern.rs](crates/sweet_test/src/wasm/js_runtime_extern.rs)

**/
globalThis.cwd = () => Deno.cwd()
globalThis.exit = (code:number) => Deno.exit(code)
globalThis.panic_to_error = f => f();
globalThis.read_file = (path: string) => {
	try {
		return Deno.readTextFileSync(path)
	} catch (err) {
		return null
	}
};
globalThis.sweet_root = () => Deno.env.get("SWEET_ROOT")

/// ⚠️ The runner will clear the console in watch mode
const wasm = await init()
.catch((err: any) => {
	// panicked!
	console.error(err);
	Deno.exit(1);
})

// may not exist and thats ok
await wasm.run_with_pending?.().catch((err: any) => {
	// panicked!
	console.error(err);
	Deno.exit(1);
})
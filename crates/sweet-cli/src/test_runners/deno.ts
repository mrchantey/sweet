// @ts-nocheck
// deno-lint-ignore-file 
// wasm-bindgen deno runner https://vscode.dev/github/rustwasm/wasm-bindgen/blob/main/crates/cli/src/bin/wasm-bindgen-test-runner/deno.rs
import init from './bindgen.js'

// This file is cached and will be replaced on hash change
// cwd will be relative to the crate, ie
// cargo test --workspace is different cwd from cargo test -p my_crate
// so we try to use absolute dir


// wrapper function to avoid abort
globalThis.panic_to_error = f => f();
globalThis.cwd = () => Deno.cwd()
globalThis.sweet_root = () => Deno.env.get("SWEET_ROOT")
globalThis.read_file = (path: string) => {
	try {
		return Deno.readTextFileSync(path)
	} catch (err) {
		return null
	}
};

/// ⚠️ The runner will clear the console, do all yer messagin afta
const wasm = await init()
.catch((err: any) => {
	console.error(err);
	Deno.exit(1);
})

// may not exist and thats ok
await wasm.run_with_pending?.().catch((err: any) => {
	console.error(err);
	Deno.exit(1);
})
// @ts-nocheck
// deno-lint-ignore-file 
// wasm-bindgen deno runner https://vscode.dev/github/rustwasm/wasm-bindgen/blob/main/crates/cli/src/bin/wasm-bindgen-test-runner/deno.rs
import init from './bindgen.js'

// THIS FILE IS CACHED
// During development, delete target/sweet/deno.ts in between changes

console.log('\nhi from deno')

// cwd will be relative to the crate, ie
// cargo test --workspace is different cwd from cargo test -p my_crate
// so we try to use absolute dir
const sweet_root = Deno.env.get('SWEET_ROOT') || Deno.cwd();
console.dir(sweet_root);

// wrapper function to avoid abort
globalThis.__wbg_test_invoke = f => f();
globalThis.read_file = (path: string) => {
	let full_path = `${sweet_root}/${path}`
	try {
		return Deno.readTextFileSync(full_path)
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
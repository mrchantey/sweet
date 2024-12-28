// @ts-nocheck
// deno-lint-ignore-file 
// for inspiration 
// https://vscode.dev/github/rustwasm/wasm-bindgen/blob/main/crates/cli/src/bin/wasm-bindgen-test-runner/deno.rs
import init from './target/wasm/bindgen.js'

globalThis.__wbg_test_invoke = f => f();

const wasm = await init()
	.catch((_err:any) => {		
		// console.error('Uncaught panic:\n', _err);
		Deno.exit(1);
	})
	

	console.log('all good me lad?')
// await globalThis['__sweet_pending_test_promises'][0]
// 	.catch(_err => { 
// 			// console.error('Uncaught panic:\n', _err);
// 		// Deno.exit(1)
// 	})
// await globalThis['__sweet_pending_test_promises'][1]
// 	.catch(_err => { 
// 			// console.error('Uncaught panic:\n', _err);
// 		// Deno.exit(1)
// 	})


await wasm.run_with_pending().catch((_err:any) => {
	console.error('Uncaught panic:\n', _err);
	Deno.exit(1);
})
console.dir(globalThis['__sweet_panic_output'])
console.dir(globalThis['__sweet_pending_test_results'])
console.dir(globalThis['__sweet_pending_test_descriptions'])
// console.dir(globalThis['__sweet_panic_output'])
// let err = 

console.log('yes me lad')
// deno-lint-ignore-file 
import init from './target/wasm/bindgen.js'

// const watch = Deno.args.includes('--watch')
const wasm = await init()
	.catch((_err:any) => {		
		// console.error('Uncaught panic:\n', _err);
		Deno.exit(1);
	})
	// .then(() => console.log('Deno: Ok'))
console.log('Deno: Ok')	

setTimeout(() => {
	console.log('🚀')	
},1000)

try {
	/* @ts-ignore */
	await globalThis['__sweet_pending_test_promises'][23]
	// await a.catch((_err: any) => {
		// 	console.log('🚀🚀🚀')
		// })
	} catch (err) {
		
	}
	
console.log('EVERYTHING IS OK')
/* @ts-ignore */
console.dir(globalThis['__sweet_panic_output'])
// let err = 

// console.dir(a.catch((err: any) => {
	
// }_

// console.log('prom'

// await wasm.run_with_pending().catch((_err:any) => {
// 	// console.error('Uncaught panic:\n', _err);
// 		Deno.exit(1);
// 	})
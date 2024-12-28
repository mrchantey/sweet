
import init from './target/wasm/bindgen.js'

// const watch = Deno.args.includes('--watch')
await init()
	.catch(_err => {		
		// console.error('Uncaught panic:\n', _err);
			Deno.exit(1);
		})
		// .then(() => console.log('Deno: Ok'))
console.log('Deno: Ok')
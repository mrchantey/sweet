
import init from './target/wasm/bindgen.js'

const watch = Deno.args.includes('--watch')

init()
	.catch(_err => {		
		// console.error('Uncaught panic:\n', _err);
		// in watch mode we dont emit errors, it just dirties the stdout
		if(!watch)
			Deno.exit(1);
	})
	// .then(() => console.log('Deno: Ok'))

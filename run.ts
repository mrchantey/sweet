
import init from './target/wasm/bindgen.js'
init()
	.catch(err => console.error('Deno: Error:\n', err))
	.then(() => console.log('Deno: Ok'))

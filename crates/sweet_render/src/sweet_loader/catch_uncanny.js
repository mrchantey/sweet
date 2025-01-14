globalThis._sweet = {
	uncanny: [],
	event: (id, event) => {
		globalThis._sweet.uncanny.push([id, event])
	}
}
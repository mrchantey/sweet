use backtrace::Backtrace;

#[cfg(target_os = "linux")]
const PREFIX_STACK_FRAMES: usize = 1;
#[cfg(target_os = "macos")]
const PREFIX_STACK_FRAMES: usize = 5;
#[cfg(target_os = "windows")]
const PREFIX_STACK_FRAMES: usize = 6;
#[cfg(not(any(
	target_os = "linux",
	target_os = "macos",
	target_os = "windows"
)))]
const PREFIX_STACK_FRAMES: usize = 0;


#[cfg(target_arch = "wasm32")]
pub fn file_context_depth(_: usize) -> String {
	String::new()
	// String::from("backtrace not yet supported in wasm..").grey()
}

#[cfg(not(target_arch = "wasm32"))]
pub fn file_context_depth(frame_depth: usize) -> String {
	let bt = Backtrace::new();
	let min_frame = PREFIX_STACK_FRAMES + frame_depth;
	let frame = &bt.frames()[min_frame..][0];
	let symbol = &frame.symbols()[0];
	if let Some(file) = crate::test_case::BacktraceFile::new(symbol) {
		return file.file_context().unwrap_or_default();
	}
	String::new()
}
pub fn file_context() -> String {
	//include this frame
	file_context_depth(1)
}

pub fn trace_all() {
	let bt = Backtrace::new();
	let frames = &bt.frames()[PREFIX_STACK_FRAMES..];
	// bt/.
	for frame in frames {
		for symbol in frame.symbols().iter() {
			if let Some(file) = symbol.filename() {
				if let Some(file) = file.to_str() {
					println!("{}", file);
					// if file.contains(RUST_INTERNAL_FILE) {
					// 	return;
					// }
					// if let Some(line) = symbol.lineno() {
					// 	println!("{}: {}", file, line);
					// }
				}
			}
		}
		// println!("{:?}", symbol);
	}
}

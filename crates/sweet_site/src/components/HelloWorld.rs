use sweet_rsx::rsx;










#[allow(non_snake_case)]
pub fn HelloWorld() {
	rsx! {
		<button onclick={|_|{alert!("hello world!")}}>Click Me</button>
	}
}

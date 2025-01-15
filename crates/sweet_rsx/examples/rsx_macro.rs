use sweet::prelude::rsx;

fn main() {
	let _foo = rsx! {
		<div onclick>
			<p>hello</p>
		</div>
	};
}

use sweet::prelude::*;


pub struct HelloWorld;

impl Rsx for HelloWorld {
	fn into_parts(self) -> RsxParts {
		// let planet = "mars";
		// rsx! {
		// 	<button
		// 		onclick={|_|{alert!(format!("hello {}!", planet))}}>
		// 			Click Me
		// 	</button>
		// }
	}
}

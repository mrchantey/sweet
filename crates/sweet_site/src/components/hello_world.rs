use sweet::prelude::*;


pub struct HelloWorld;

impl IntoRsx for HelloWorld {
	fn into_rsx(self) -> impl Rsx {
		let planet = "mars";
		// rsx! {
		// 	<button
		// 		onclick={|_|{alert!(format!("hello {}!", planet))}}>
		// 			Click Me
		// 	</button>
		// }
	}
}

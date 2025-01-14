use sweet::prelude::*;


pub struct HelloWorld;

impl Component for HelloWorld {
	fn render(self) -> impl Rsx {
		// let planet = "mars";
		// rsx! {
		// 	<button
		// 		onclick={|_|{alert!(format!("hello {}!", planet))}}>
		// 			Click Me
		// 	</button>
		// }
	}
}

use bevy::prelude::*;
use sweet_core as sweet;
use sweet_core::prelude::Component;
use sweet_core::prelude::*;
use sweet_rsx_macros::rsx;


fn main() {
	let mut app = App::new();
	app.run();
}



struct MyComponent {
	initial_value: i32,
}

impl Component for MyComponent {
	fn render(self) -> impl Rsx {
		rsx! {
			<div>
				{self.initial_value}
			</div>
		}
	}
}

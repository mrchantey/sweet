use sweet_core::prelude::*;


#[derive(Default)]
pub struct Server;

impl Server {
	// pub fn run(self, rsx: impl Component) {
	// 	// let rsx = rsx.render();
	// 	// println!("todo run server for {:?}", rsx);
	// 	todo!()
	// }

	pub fn run_once(self, rsx: impl Rsx) {
		let parts = rsx.into_rsx_parts();
		println!("todo run server for {:?}", parts);
	}
}

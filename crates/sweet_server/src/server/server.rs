use sweet_core::prelude::*;


#[derive(Default)]
pub struct Server;

impl Server {
	pub fn run(self, rsx: impl Rsx) {
		let rsx = rsx.into_parts();
		println!("todo run server for {:?}", rsx);
	}

	pub fn run_once(self, rsx: impl Rsx) {
		let rsx = rsx.into_parts().into_parts();

		println!("todo run server for {:?}", rsx);
	}
}

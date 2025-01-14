use sweet_core::prelude::*;


#[derive(Default)]
pub struct Server;

impl Server {
	pub fn run(self, rsx: impl Rsx) {
		let rsx = rsx.into_parts();

		todo!("todo run server for {:?}", rsx);
	}
}

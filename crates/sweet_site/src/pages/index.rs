pub use super::super::components::counter;
pub use sweet_rsx::rsx;
use sweet_rsx::rsx::IntoRsx;



pub struct Index;

// maybe like IntoRoute or something
impl IntoRsx for Index {
	fn into_rsx(self) {
		let initial_value = 200;
		// rsx! {
		// 	<h1>Counter</h1>
		// 	<Counter initial_value/>
		// }
	}
}

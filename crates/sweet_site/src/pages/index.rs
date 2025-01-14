pub use super::super::components::counter;
use sweet::prelude::*;
pub struct Index;

// maybe like IntoRoute or something
impl Rsx for Index {
	fn into_parts(self) -> RsxParts {
		let initial_value = 200;
		// rsx! {
		// 	<h1>Counter</h1>
		// 	<Counter initial_value/>
		// }
	}
}

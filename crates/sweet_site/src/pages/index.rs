pub use crate::components::Counter;
pub use sweet_rsx::rsx;

pub fn route() {
	let initial_value = 200;

	rsx! {
		<h1>Counter</h1>
		<Counter initial_value/>
	}
}

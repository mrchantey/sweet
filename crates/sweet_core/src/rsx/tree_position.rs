use anyhow::anyhow;
use anyhow::Result;
/// Represents the position of a node in the tree.
/// This always has at least one element.
///
/// Considering the following:
/// ```html
/// <html data-sweet-pos="0">
/// 	<head data-sweet-pos="0,0"></head>
/// 	<body data-sweet-pos="0,1">
/// 		<div data-sweet-pos="0,1,0"></div>
/// 		<div data-sweet-pos="0,1,1"></div>
/// 	</body>
/// </html>
/// ```
///
///
#[derive(Debug, Default, Clone, Hash, PartialEq, Eq)]
pub struct TreePosition(Vec<usize>);

impl TreePosition {
	pub fn new() -> Self { Self::default() }
	/// if vec is empty, returns a new TreePosition with a single 0
	pub fn from_vec(vec: Vec<usize>) -> Self { Self(vec) }
	/// # Panics
	/// if there are no positions
	pub fn next_sibling(&mut self) { *self.0.last_mut().unwrap() += 1; }
	/// # Panics
	/// if there are no positions, or if the last position is already 0
	pub fn prev_sibling(&mut self) { *self.0.last_mut().unwrap() -= 1; }
	pub fn push_child(&mut self) { self.0.push(0); }
	/// # Panics
	/// if there are no positions
	pub fn pop_child(&mut self) { self.0.pop(); }
	/// Convert to a comma separated value string
	/// ie "0,1,2"
	pub fn to_csv(&self) -> String {
		self.0
			.iter()
			.map(|i| i.to_string())
			.collect::<Vec<String>>()
			.join(",")
	}

	/// Tree position from comma separated values, ie "0,1,2"
	/// # Errors
	/// - if any of the values are not parsable as usize
	pub fn from_csv(csv: &str) -> anyhow::Result<Self> {
		let values = csv
			.split(",")
			.map(|s| {
				s.parse().map_err(|e| {
					anyhow!("failed to parse csv for TreePosition: {s}\n{}", e)
				})
			})
			.collect::<Result<Vec<_>>>()?;
		Ok(Self(values))
	}
}


// impl std::ops::Deref for TreePosition {
// 	type Target = Vec<usize>;
// 	fn deref(&self) -> &Self::Target { &self.0 }
// }

// impl std::ops::DerefMut for TreePosition {
// 	fn deref_mut(&mut self) -> &mut Self::Target { &mut self.0 }
// }


#[cfg(test)]
mod test {
	use crate::prelude::*;
	// use sweet_rsx::prelude::*;
	// use sweet_test::prelude::*;

	#[test]
	fn works() {
		let pos = TreePosition::default();

		assert_eq!(pos.0, vec![]);
	}
}

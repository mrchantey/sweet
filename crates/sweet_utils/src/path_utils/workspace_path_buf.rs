use path_clean::PathClean;
use std::path::Path;
use std::path::PathBuf;
use std::str::FromStr;






/// A newtype `PathBuf` with several requirements:
/// 1. the user pinky swears that the path is relative to the workspace root
/// 2. the path is cleaned using [`path_clean`]
/// 3. on windows backslashes are replaced by forward slashes
///
/// The path does **not** have to exist
///
/// ## Example
///
/// ```rust
/// # use sweet_utils::prelude::*;
/// let path = WorkspacePathBuf::new(file!());
///
/// ```
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
pub struct WorkspacePathBuf(PathBuf);


impl WorkspacePathBuf {
	/// Create a new [`WorkspacePathBuf`], a common use case is to use `file!()`
	/// which is already relative to the workspace root.
	pub fn new(path: impl AsRef<Path>) -> Self {
		let path = path.as_ref().clean();
		#[cfg(target_os = "windows")]
		{
			let path = path.to_string_lossy().replace('\\', "/");
			Self(PathBuf::from(path))
		}
		#[cfg(not(target_os = "windows"))]
		{
			Self(path)
		}
	}

	/// Using calls like `std::fs::read_dir` will return paths relative
	/// to the current directory of the process, not the workspace root.
	/// This function will resolve the difference by first canonicalizing
	/// the path and then stripping the workspace root.
	/// ## Panics
	///
	/// Panics if [`FsExt::workspace_root`](super::FsExt::workspace_root) fails.
	#[cfg(not(target_arch = "wasm32"))]
	pub fn new_from_current_directory(
		path: impl AsRef<Path>,
	) -> anyhow::Result<Self> {
		use super::FsExt;
		use crate::prelude::PathExt;
		let path = PathExt::canonicalize(path)?;
		let workspace_root = FsExt::workspace_root();
		let path = path.strip_prefix(workspace_root)?;
		Ok(Self::new(path))
	}

	#[cfg(not(target_arch = "wasm32"))]
	/// Convert to a [`CanonicalPathBuf`]
	pub fn into_canonical(&self) -> super::FsResult<super::CanonicalPathBuf> {
		let path = super::FsExt::workspace_root().join(self);
		let canonical = super::CanonicalPathBuf::new(path)?;
		Ok(canonical)
	}
}

impl std::ops::Deref for WorkspacePathBuf {
	type Target = PathBuf;
	fn deref(&self) -> &Self::Target { &self.0 }
}
impl AsRef<Path> for WorkspacePathBuf {
	fn as_ref(&self) -> &Path { self.0.as_ref() }
}
impl FromStr for WorkspacePathBuf {
	type Err = anyhow::Error;
	fn from_str(s: &str) -> Result<Self, Self::Err> { Ok(Self::new(s)) }
}



#[cfg(test)]
mod test {
	use crate::prelude::*;
	use std::path::PathBuf;

	#[test]
	fn works() {
		assert_eq!(
			WorkspacePathBuf::new("Cargo.toml").as_path(),
			PathBuf::from("Cargo.toml").as_path()
		);
		assert_eq!(
			WorkspacePathBuf::new("foo/../Cargo.toml").as_path(),
			PathBuf::from("Cargo.toml").as_path()
		);
	}
}

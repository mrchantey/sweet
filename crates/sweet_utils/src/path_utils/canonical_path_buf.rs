use super::FsError;
use super::FsExt;
use super::FsResult;
use super::PathExt;
use path_clean::PathClean;
use std::path::Path;
use std::path::PathBuf;
use std::str::FromStr;

/// Wrapper for `CanonicalPathBuf::new_workspace_rel(file!())`,
/// for use as a drop-in replacement for `file!()`.
/// ## Example
///
/// ```rust
/// # use sweet_utils::prelude::*;
/// let path = canonical_file!();
/// ```
#[macro_export]
macro_rules! canonical_file {
	() => {
		CanonicalPathBuf::new_workspace_rel(file!()).unwrap()
	};
}



/// A newtype `PathBuf` that makes several guarantees:
/// 1. the path is canonical
/// 2. on windows backslashes are replaced by forward slashes
/// 3. The hash is cross-platform as it uses encoded bytes
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CanonicalPathBuf(PathBuf);

impl Default for CanonicalPathBuf {
	fn default() -> Self {
		Self::new(std::env::current_dir().unwrap()).unwrap()
	}
}

impl CanonicalPathBuf {
	/// Create a new [`CanonicalPathBuf`] from a `PathBuf`.
	/// Canonicalization will prepend the `env::current_dir`,
	/// if your path is instead relative to the workspace root, ie `file!()`,
	/// use [`CanonicalPathBuf::new_workspace_rel`].
	///
	/// For wasm builds this just return the path as is.
	///
	/// ## Panics
	/// Panics if the path cannot be canonicalized. This will always be the case
	/// for wasm builds or if the path does not exist.
	///
	/// ## Example
	///
	/// ```rust
	/// # use sweet_utils::prelude::*;
	/// let path = CanonicalPathBuf::new("Cargo.toml");
	/// ```
	pub fn new(path: impl AsRef<Path>) -> FsResult<Self> {
		#[cfg(target_os = "windows")]
		{
			let canonical = PathExt::canonicalize(path)?;
			let canonical =
				canonical.to_string_lossy().replace('\\', "/").to_path_buf();
			Ok(Self(canonical))
		}
		#[cfg(not(target_os = "windows"))]
		{
			Ok(Self(PathExt::canonicalize(path)?))
		}
	}
	/// Create a new [`CanonicalPathBuf`] from a path relative to the workspace root,
	/// ie from using the `file!()` macro.
	pub fn new_workspace_rel(path: impl AsRef<Path>) -> FsResult<Self> {
		let path = FsExt::workspace_root().join(path);
		Self::new(path)
	}

	pub fn new_unchecked(path: impl AsRef<Path>) -> Self {
		let path = path.as_ref().clean();
		#[cfg(target_os = "windows")]
		{
			let canonical =
				path.to_string_lossy().replace('\\', "/").to_path_buf();
			Ok(Self(canonical))
		}
		Self(path)
	}
}
impl FromStr for CanonicalPathBuf {
	type Err = FsError;
	fn from_str(val: &str) -> Result<Self, Self::Err> { Self::new(val) }
}

impl AsRef<Path> for CanonicalPathBuf {
	fn as_ref(&self) -> &Path { self.0.as_ref() }
}

impl std::ops::Deref for CanonicalPathBuf {
	type Target = PathBuf;

	fn deref(&self) -> &Self::Target { &self.0 }
}


#[cfg(test)]
#[cfg(not(target_arch = "wasm32"))]
mod test {
	use crate::prelude::*;
	// use sweet_test::prelude::*;

	#[test]
	fn works() {
		let _buf = CanonicalPathBuf::new("Cargo.toml").unwrap();
		let buf1 = CanonicalPathBuf::new(FsExt::workspace_root().join(file!()))
			.unwrap();
		let buf2 = canonical_file!();
		assert_eq!(buf1, buf2);
		assert!(buf1.to_string_lossy().ends_with("canonical_path_buf.rs"));
	}
}

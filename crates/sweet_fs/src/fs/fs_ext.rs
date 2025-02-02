use crate::prelude::*;
use std::ffi::OsString;
use std::fs;
use std::path::Path;
use std::path::PathBuf;
use sweet_utils::prelude::PathExt;

/// Better Fs, actually outputs missing path
pub struct FsExt;

impl FsExt {
	/// Copy a directory recursively, creating it if it doesnt exist
	/// This also provides consistent behavior with the `cp` command:
	/// -
	pub fn copy_recursive(
		source: impl AsRef<Path>,
		destination: impl AsRef<Path>,
	) -> FsResult<()> {
		let source = source.as_ref();
		let destination = destination.as_ref();

		fs::create_dir_all(&destination).ok();
		for entry in ReadDir::files(source)? {
			let stem = PathExt::file_stem(&entry)?;
			if entry.is_dir() {
				Self::copy_recursive(&entry, destination.join(stem))?;
			} else {
				fs::copy(&entry, destination.join(stem))
					.map_err(FsError::from_io)?;
			}
		}
		Ok(())
	}

	/// remove a directory and all its contents
	pub fn remove(path: impl AsRef<Path>) -> FsResult<()> {
		fs::remove_dir_all(path).map_err(FsError::from_io)?;
		Ok(())
	}


	// pub fn dir_contains(path: PathBuf, pattern: &str) -> bool {
	// 	let pattern = Pattern::new(pattern).unwrap();
	// 	glob::glob_with(
	// 		&pattern.to_string(),
	// 		glob::MatchOptions {
	// 			case_sensitive: false,
	// 			require_literal_separator: false,
	// 			require_literal_leading_dot: false,
	// 		},
	// 	)
	// 	read_dir_recursive(path)
	// 		.iter()
	// 		.any(|p| pattern. p.to_str().unwrap().contains(pattern))
	// }

	/// Return the closest ancestor (inclusive) that contains a `Cargo.lock` file
	/// # Panics
	/// - The current directory is not found
	/// - Insufficient permissions to access the current directory
	/// - There is no `Cargo.lock` in the directory or any of its ancestors
	pub fn workspace_root() -> PathBuf {
		let path = std::env::current_dir().unwrap();
		let mut path_ancestors = path.as_path().ancestors();
		while let Some(p) = path_ancestors.next() {
			if ReadDir::files(p)
				.unwrap_or_default()
				.into_iter()
				.any(|p| p.file_name() == Some(&OsString::from("Cargo.lock")))
			{
				return PathBuf::from(p);
			}
		}
		panic!("No Cargo.lock found in the current directory or any of its ancestors");
	}


	/// Write a file, ensuring the path exists
	pub fn write(path: impl AsRef<Path>, data: &str) -> FsResult<()> {
		if let Some(parent) = path.as_ref().parent() {
			fs::create_dir_all(parent)?;
		}
		fs::write(path, data)?;
		Ok(())
	}
}

#[cfg(test)]
impl FsExt {
	pub fn test_dir() -> PathBuf {
		Self::workspace_root().join(Path::new("crates/sweet_fs/test_dir"))
	}
}


#[cfg(test)]
mod test {
	use super::FsExt;

	#[test]
	fn workspace_root() {
		assert_eq!(
			FsExt::workspace_root()
				.file_stem()
				.unwrap()
				.to_str()
				.unwrap(),
			"sweet"
		);
		assert!(FsExt::workspace_root().join("Cargo.lock").exists());
	}
}

use super::FsError;
use super::FsExt;
use super::FsResult;
use std::ffi::OsStr;
use std::path::Path;
use std::path::PathBuf;

pub struct PathExt;

impl PathExt {
	/// Create a path relative to the current working directory
	pub fn relative(path: &impl AsRef<Path>) -> FsResult<&Path> {
		let cwd = FsExt::current_dir()?;
		PathExt::strip_prefix(path, &cwd)
	}

	/// Strip prefix
	pub fn strip_prefix<'a>(
		path: &'a impl AsRef<Path>,
		prefix: &impl AsRef<Path>,
	) -> FsResult<&'a Path> {
		path.as_ref()
			.strip_prefix(prefix)
			.map_err(|e| FsError::other(path.as_ref(), e))
	}

	/// Create an absolute path from a relative path
	pub fn canonicalize(path: impl AsRef<Path>) -> FsResult<PathBuf> {
		let path = path.as_ref();
		path.canonicalize().map_err(|e| FsError::io(path, e))
	}
	pub fn to_forward_slash(path: impl AsRef<Path>) -> PathBuf {
		Path::new(&path.as_ref().to_string_lossy().replace("\\", "/"))
			.to_path_buf()
	}
	pub fn to_forward_slash_str(path: impl AsRef<Path>) -> String {
		path.as_ref()
			.to_str()
			.unwrap_or_default()
			.replace("\\", "/")
	}

	pub fn file_stem(path: &impl AsRef<Path>) -> FsResult<&OsStr> {
		let path = path.as_ref();
		path.file_stem()
			.ok_or_else(|| FsError::other(path, "No file stem"))
	}
	pub fn is_dir_or_extension(path: &impl AsRef<Path>, ext: &str) -> bool {
		let path = path.as_ref();
		match path.extension() {
			Some(value) => value.to_str().unwrap() == ext,
			None => path.is_dir(),
		}
	}
}

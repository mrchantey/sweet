use clap::Parser;
use std::path::Path;
use std::path::PathBuf;
use sweet::prelude::*;



#[derive(Debug, Default, Clone, Parser)]
#[command(name = "mod")]
pub struct AutoMod {
	#[command(flatten)]
	pub watcher: FsWatcher,

	#[arg(short, long)]
	pub quiet: bool,
}


impl AutoMod {
	pub async fn run(&self) -> Result<()> {
		self.watcher.assert_path_exists()?;
		self.watcher
			.watch_async(|e| {
				e.events
					.iter()
					.filter(|e| self.should_run(e))
					.map(|e| self.handle_event(e))
					.collect::<Result<Vec<_>>>()?;
				Ok(())
			})
			.await?;
		Ok(())
	}

	fn should_run(&self, e: &WatchEvent) -> bool {
		if e.path.extension() == Some("rs".as_ref())
			&& matches!(e.kind, EventKind::Create(_))
		{
			true
		} else {
			false
		}
	}

	fn handle_event(&self, e: &WatchEvent) -> Result<()> {
		let Ok((parent_mod, mod_file)) = Self::path_to_str(&e.path) else {
			return Ok(());
		};
		FsExt::write(&parent_mod, &mod_file)?;
		if !self.quiet {
			println!("AutoMod: updated {}", parent_mod.display());
		}
		Ok(())
	}

	fn path_to_str(path: impl AsRef<Path>) -> Result<(PathBuf, String)> {
		let path = path.as_ref();
		let Some(parent) = path.parent() else {
			anyhow::bail!("No parent found for path {}", path.display());
		};
		let is_lib_dir =
			parent.file_name().map(|f| f == "lib").unwrap_or(false);

		let parent_mod = if is_lib_dir {
			parent.join("lib.rs")
		} else {
			parent.join("mod.rs")
		};

		let Some(file_stem) = path.file_stem().map(|s| s.to_string_lossy())
		else {
			anyhow::bail!("No file stem found for path {}", path.display());
		};

		let mut mod_file = ReadFile::to_string(&parent_mod).unwrap_or_default();

		let vis = if is_lib_dir { "pub " } else { "" };

		let mut new_mod_file = format!("{vis}mod {file_stem};");

		if is_lib_dir {
			mod_file = mod_file.replace(
				"pub mod prelude {",
				"pub mod prelude {\npub use crate::{file_stem}::*;",
			);
		} else {
			new_mod_file.push_str(&format!("pub use {file_stem}::*;"));
		}
		new_mod_file.push_str(&mod_file);

		Ok((parent_mod, new_mod_file))
	}
}


#[cfg(test)]
mod test {
	use crate::prelude::*;
	use sweet::prelude::*;

	#[test]
	fn works() {
		expect(AutoMod::path_to_str("foo/bar.rs").unwrap().1)
			.to_be("mod bar;pub use bar::*;");
		expect(
			AutoMod::path_to_str(CanonicalPathBuf::new_unchecked(
				FsExt::workspace_root().join("crates/sweet-cli/src/foo.rs"),
			))
			.unwrap()
			.1,
		)
		.to_be("mod foo;pub use foo::*;");
	}
}

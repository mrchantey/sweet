use clap::Parser;
use quote::quote;
use rapidhash::RapidHashMap;
use std::path::Path;
use std::path::PathBuf;
use sweet::fs::exports::notify::EventKind;
use sweet::fs::exports::notify::event::ModifyKind;
use sweet::fs::exports::notify::event::RenameMode;
use sweet::prelude::*;
use syn::File;
use syn::Ident;
use syn::ItemMod;
use syn::ItemUse;
use syn::UseTree;


#[derive(Debug, Default, Clone, Parser)]
#[command(name = "mod")]
pub struct AutoMod {
	#[command(flatten)]
	pub watcher: FsWatcher,

	#[arg(short, long)]
	pub quiet: bool,
}


impl AutoMod {
	pub async fn run(mut self) -> Result<()> {
		self.watcher.assert_path_exists()?;
		if !self.quiet {
			println!(
				"🤘 sweet as 🤘\nWatching for file changes in {}",
				self.watcher.cwd.canonicalize()?.display()
			);
		}

		self.watcher.infallible = true;
		self.watcher.filter = self
			.watcher
			.filter
			.with_exclude("**/mod.rs")
			.with_include("**/*.rs");
		self.watcher
			.watch_async(|e| {
				let mut files = ModFiles::default();
				e.events
					.iter()
					.map(|e| self.handle_event(&mut files, e))
					.collect::<Result<Vec<_>>>()?;
				files.write_all()?;
				Ok(())
			})
			.await?;
		Ok(())
	}

	fn handle_event(&self, files: &mut ModFiles, e: &WatchEvent) -> Result<()> {
		enum Step {
			Insert,
			Remove,
		}

		// let (parent_mod, mod_file) = Self::insert_mod(&e.path)?;
		// self.write_file("insert", &e.path, parent_mod, mod_file)?;

		let step = match e.kind {
			EventKind::Create(_)
			| EventKind::Modify(ModifyKind::Name(RenameMode::To)) => Step::Insert,
			EventKind::Remove(_)
			| EventKind::Modify(ModifyKind::Name(RenameMode::From)) => Step::Remove,
			EventKind::Modify(ModifyKind::Name(RenameMode::Both)) => {
				if e.path.exists() {
					Step::Insert
				} else {
					Step::Remove
				}
			}
			_ => {
				return Ok(());
			}
		};

		let file_meta = FileMeta::new(&e.path)?;
		let file = files.get_mut(&file_meta.parent_mod)?;
		match step {
			Step::Insert => {
				Self::insert_mod(file, file_meta)?;
				if !self.quiet {
					println!(
						"AutoMod: insert {}",
						PathExt::relative(&e.path)?.display(),
					);
				}
			}
			Step::Remove => {
				Self::remove_mod(file, file_meta)?;
				if !self.quiet {
					println!(
						"AutoMod: remove {}",
						PathExt::relative(&e.path)?.display(),
					);
				}
			}
		}


		Ok(())
	}

	/// Load the parents `mod.rs` or `lib.rs` file and insert a new module
	fn insert_mod(
		mod_file: &mut File,
		FileMeta {
			is_lib_dir,
			parent_mod,
			file_stem,
			mod_ident,
			..
		}: FileMeta,
	) -> Result<()> {
		for item in &mut mod_file.items {
			if let syn::Item::Mod(m) = item {
				if m.ident == file_stem {
					anyhow::bail!(
						"Module {} already exists in {}",
						file_stem,
						parent_mod.display()
					);
				}
			}
		}

		let vis = if is_lib_dir {
			quote! {pub}
		} else {
			Default::default()
		};


		let insert_pos = mod_file
			.items
			.iter()
			.position(|item| matches!(item, syn::Item::Mod(_)))
			.unwrap_or(mod_file.items.len());

		let mod_def: ItemMod = syn::parse_quote!(#vis mod #mod_ident;);
		mod_file.items.insert(insert_pos, mod_def.into());

		if is_lib_dir {
			// export in prelude
			for item in &mut mod_file.items {
				if let syn::Item::Mod(m) = item {
					if m.ident == "prelude" {
						if let Some(content) = m.content.as_mut() {
							content.1.push(
								syn::parse_quote!(pub use crate::#mod_ident::*;),
							);
						} else {
							m.content =
								Some((syn::token::Brace::default(), vec![
									syn::parse_quote!(pub use crate::#mod_ident::*;),
								]));
						}
						break;
					}
				}
			}
		} else {
			// export at root
			mod_file.items.insert(
				insert_pos + 1,
				syn::parse_quote!(pub use #mod_ident::*;),
			);
		}

		Ok(())
	}

	fn remove_mod(
		mod_file: &mut File,
		FileMeta {
			is_lib_dir,
			parent_mod,
			file_stem,
			mod_ident,
			..
		}: FileMeta,
	) -> Result<(PathBuf, String)> {
		mod_file.items.retain(|item| {
			if let syn::Item::Mod(m) = item {
				if m.ident == file_stem {
					return false;
				}
			}
			true
		});

		// Remove the re-export
		if is_lib_dir {
			// Remove from prelude
			for item in &mut mod_file.items {
				if let syn::Item::Mod(m) = item {
					if m.ident == "prelude" {
						if let Some(content) = m.content.as_mut() {
							content.1.retain(|item| {
								if let syn::Item::Use(use_item) = item {
									if let Some(last) = use_item_ident(use_item)
									{
										return last != &mod_ident;
									}
								}
								true
							});
						}
						break;
					}
				}
			}
		} else {
			// Remove re-export at root
			mod_file.items.retain(|item| {
				if let syn::Item::Use(use_item) = item {
					if let Some(last) = use_item_ident(use_item) {
						return last != &mod_ident;
					}
				}
				true
			});
		}

		Ok((parent_mod, prettyplease::unparse(&mod_file)))
	}
}
/// find the first part of an ident, skiping `crate`, `super` or `self`
fn use_item_ident(use_item: &ItemUse) -> Option<&Ident> {
	const SKIP: [&str; 3] = ["crate", "super", "self"];
	match &use_item.tree {
		UseTree::Path(use_path) => {
			if SKIP.contains(&use_path.ident.to_string().as_str()) {
				match &*use_path.tree {
					UseTree::Path(use_path) => {
						return Some(&use_path.ident);
					}
					UseTree::Name(use_name) => {
						return Some(&use_name.ident);
					}
					_ => {}
				}
			} else {
				return Some(&use_path.ident);
			}
		}
		_ => {}
	}
	None
}

#[derive(Default, Clone)]
struct ModFiles {
	map: RapidHashMap<PathBuf, File>,
}

impl ModFiles {
	pub fn get_mut(&mut self, path: impl AsRef<Path>) -> Result<&mut File> {
		let path = path.as_ref();
		if !self.map.contains_key(path) {
			let file = ReadFile::to_string(path)?;
			let file = syn::parse_file(&file)?;
			self.map.insert(path.to_path_buf(), file);
		}
		Ok(self.map.get_mut(path).unwrap())
	}
	pub fn write_all(&self) -> Result<()> {
		for (path, file) in &self.map {
			let file = prettyplease::unparse(file);
			FsExt::write(path, &file)?;
			println!("AutoMod: write {}", path.display());
		}
		Ok(())
	}
}

struct FileMeta<'a> {
	pub is_lib_dir: bool,
	pub parent_mod: PathBuf,
	pub file_stem: String,
	#[allow(dead_code)]
	pub event_path: &'a Path,
	pub mod_ident: syn::Ident,
}

impl<'a> FileMeta<'a> {
	/// Returns either `lib.rs` or `mod.rs` for the given path's parent
	fn new(event_path: &'a Path) -> Result<Self> {
		let Some(parent) = event_path.parent() else {
			anyhow::bail!("No parent found for path {}", event_path.display());
		};
		let is_lib_dir =
			parent.file_name().map(|f| f == "src").unwrap_or(false);
		let parent_mod = if is_lib_dir {
			parent.join("lib.rs")
		} else {
			parent.join("mod.rs")
		};
		let Some(file_stem) = event_path
			.file_stem()
			.map(|s| s.to_string_lossy().to_string())
		else {
			anyhow::bail!(
				"No file stem found for path {}",
				event_path.display()
			);
		};

		let mod_ident =
			syn::Ident::new(&file_stem, proc_macro2::Span::call_site());

		Ok(Self {
			event_path,
			is_lib_dir,
			parent_mod,
			file_stem,
			mod_ident,
		})
	}
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn insert_works() {
		fn insert(workspace_path: impl AsRef<Path>) -> Result<String> {
			let canonical = CanonicalPathBuf::new_unchecked(
				FsExt::workspace_root().join(workspace_path.as_ref()),
			);
			let file_meta = FileMeta::new(canonical.as_ref())?;
			let file = ReadFile::to_string(&file_meta.parent_mod)?;
			let mut file = syn::parse_file(&file)?;
			AutoMod::insert_mod(&mut file, file_meta)?;
			let file = prettyplease::unparse(&file);
			Ok(file)
		}

		let insert_lib = insert("crates/sweet-cli/src/foo.rs").unwrap();
		expect(&insert_lib).to_contain("pub mod foo;");
		expect(&insert_lib).to_contain("pub use crate::foo::*;");

		let insert_mod = insert("crates/sweet-cli/src/bench/foo.rs").unwrap();
		expect(&insert_mod).to_contain("mod foo;");
		expect(&insert_mod).to_contain("pub use foo::*;");
	}
	#[test]
	fn remove_works() {
		fn remove(workspace_path: impl AsRef<Path>) -> Result<String> {
			let canonical = CanonicalPathBuf::new_unchecked(
				FsExt::workspace_root().join(workspace_path.as_ref()),
			);
			let file_meta = FileMeta::new(canonical.as_ref())?;
			let file = ReadFile::to_string(&file_meta.parent_mod)?;
			let mut file = syn::parse_file(&file)?;
			AutoMod::remove_mod(&mut file, file_meta)?;
			let file = prettyplease::unparse(&file);
			Ok(file)
		}

		let remove_lib = remove("crates/sweet-cli/src/automod").unwrap();
		expect(&remove_lib).not().to_contain("pub mod automod;");
		expect(&remove_lib)
			.not()
			.to_contain("pub use crate::automod::*;");


		let remove_mod =
			remove("crates/sweet-cli/src/bench/bench_assert.rs").unwrap();
		expect(&remove_mod)
			.not()
			.to_contain("pub mod bench_assert;");
		expect(&remove_mod)
			.not()
			.to_contain("pub use bench_assert::*;");
	}
}

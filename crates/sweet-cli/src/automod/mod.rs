use clap::Parser;
use quote::quote;
use std::path::Path;
use std::path::PathBuf;
use sweet::prelude::*;
use syn::ItemMod;


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
			parent.file_name().map(|f| f == "src").unwrap_or(false);

		let parent_mod = if is_lib_dir {
			parent.join("lib.rs")
		} else {
			parent.join("mod.rs")
		};

		let Some(file_stem) = path.file_stem().map(|s| s.to_string_lossy())
		else {
			anyhow::bail!("No file stem found for path {}", path.display());
		};

		let new_mod_ident =
			syn::Ident::new(&file_stem, proc_macro2::Span::call_site());

		let mod_file = ReadFile::to_string(&parent_mod).unwrap_or_default();
		let mut mod_file = syn::parse_file(&mod_file)?;
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

		let mod_def: ItemMod = syn::parse_quote!(#vis mod #new_mod_ident;);
		mod_file.items.insert(insert_pos, mod_def.into());

		if is_lib_dir {
			// export in prelude
			for item in &mut mod_file.items {
				if let syn::Item::Mod(m) = item {
					if m.ident == "prelude" {
						if let Some(content) = m.content.as_mut() {
							content.1.push(
								syn::parse_quote!(pub use crate::#new_mod_ident::*;),
							);
						} else {
							m.content =
								Some((syn::token::Brace::default(), vec![
									syn::parse_quote!(pub use crate::#new_mod_ident::*;),
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
				syn::parse_quote!(pub use #new_mod_ident::*;),
			);
		}

		let mod_file = prettyplease::unparse(&mod_file);
		Ok((parent_mod, mod_file))
	}
}


#[cfg(test)]
mod test {
	use crate::prelude::*;
	use sweet::prelude::*;

	#[test]
	fn works() {
		expect(AutoMod::path_to_str("foo/bar.rs").unwrap().1)
			.to_be("mod bar;\npub use bar::*;\n");

		let lib_update = AutoMod::path_to_str(CanonicalPathBuf::new_unchecked(
			FsExt::workspace_root().join("crates/sweet-cli/src/foo.rs"),
		))
		.unwrap()
		.1;
		expect(&lib_update).to_contain("pub mod foo;");
		expect(&lib_update).to_contain("pub use crate::foo::*;");
	}
}

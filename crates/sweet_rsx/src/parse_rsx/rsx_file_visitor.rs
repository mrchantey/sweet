use crate::prelude::*;
use syn::visit_mut;
use syn::visit_mut::VisitMut;


/// The rsx visitor is used by file (preprocessor) parsers.
pub struct RsxFileVisitor<'a, P> {
	plugin: &'a mut P,
	/// The rsx macros found in the function
	macros: Vec<WalkNodesOutput>,
	/// Errors that occurred while parsing the rsx macro
	errors: Vec<syn::Error>,
}

/// Output from a fully parsed file with multiple rsx macros.
pub struct RsxFileVisitorOut {
	pub macros: Vec<WalkNodesOutput>,
	pub errors: Vec<syn::Error>,
}

impl<'a, P> Into<RsxFileVisitorOut> for RsxFileVisitor<'a, P> {
	fn into(self) -> RsxFileVisitorOut {
		RsxFileVisitorOut {
			macros: self.macros,
			errors: self.errors,
		}
	}
}

impl<'a, P: RsxPlugin> RsxFileVisitor<'a, P> {
	pub fn new(plugin: &'a mut P) -> Self {
		Self {
			plugin,
			macros: Vec::new(),
			errors: Vec::new(),
		}
	}
	pub fn extend_result(
		&mut self,
		result: Result<RsxFileVisitorOut, syn::Error>,
	) {
		match result {
			Ok(out) => {
				self.macros.extend(out.macros);
				self.errors.extend(out.errors);
			}
			Err(e) => self.errors.push(e),
		}
	}
}

impl<'a, P: RsxPlugin> VisitMut for RsxFileVisitor<'a, P> {
	fn visit_macro_mut(&mut self, item: &mut syn::Macro) {
		if P::path_matches(&item.path) {
			match self.plugin.parse_rsx(&mut item.tokens) {
				Ok(parts) => self.macros.push(parts),
				Err(e) => self.errors.push(e),
			}
			// place path::to::rsx! with noop!
			item.path = syn::parse_quote!(sweet::noop)
		}
		// visit nested
		visit_mut::visit_macro_mut(self, item);
	}
}

use crate::prelude::*;
use syn::visit_mut;
use syn::visit_mut::VisitMut;
use syn::Expr;
use syn::ExprMacro;



/// The rsx visitor is used by file (preprocessor) parsers.
pub struct RsxFileVisitor<'a, P> {
	plugin: &'a mut P,
	/// The rsx macros found in the function
	macros: Vec<RsxPartsTokens>,
	/// Errors that occurred while parsing the rsx macro
	errors: Vec<syn::Error>,
}

pub struct RsxFileVisitorOut {
	pub macros: Vec<RsxPartsTokens>,
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
}

impl<'a, P: RsxPlugin> VisitMut for RsxFileVisitor<'a, P> {
	// TODO visit mut
	fn visit_expr_mut(&mut self, expr: &mut Expr) {
		// First visit any child nodes
		visit_mut::visit_expr_mut(self, expr);
		if let Expr::Macro(ExprMacro { mac, .. }) = expr {
			if P::path_matches(&mac.path) {
				// let parts = RsxPartsTokens::from_macro(node.mac.tokens.clone());
				if let Err(err) = self.plugin.parse_rsx(expr) {
					self.errors.push(err);
				}
			}
		}
	}
}

// #[cfg(test)]
// mod test {
// 	use crate::prelude::*;
// 	use sweet::prelude::*;

// 	#[test]
// 	fn test_extract_rsx() {
// 		let input = syn::parse_quote! {{
// 			let a = rsx!{ <div> };
// 			let b = rsx!{ <button> };
// 			}
// 		};

// 		let contents = RsxVisitor::extract_parts(&input);
// 		expect(contents.len()).to_be(2);
// 		expect(&contents[0].html).to_be("<div></div>");
// 		expect(&contents[1].html).to_be("<button></button>");
// 	}
// }

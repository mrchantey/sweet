use super::RsxFileVisitor;
use super::RsxFileVisitorOut;
use super::RsxPartsTokens;
use syn::visit_mut::VisitMut;
use syn::Expr;





pub trait RsxPlugin: Sized {
	/// specify the identifier that will be parsed
	fn macro_ident() -> &'static str { "rsx" }

	/// Check if a path matches the macro, by default only the last segment is checked
	fn path_matches(path: &syn::Path) -> bool {
		path.segments
			.last()
			.map_or(false, |seg| seg.ident == Self::macro_ident())
	}

	/// Parse a matching macro. This is the most low level call, the full expression is provided
	/// so the macro can be replaced but it has already been checked
	/// so use [unwrap_macro] with confidence
	fn parse_rsx(&mut self, item: &mut Expr) -> syn::Result<RsxPartsTokens>;

	fn parse_file(&mut self, file: &str) -> syn::Result<RsxFileVisitorOut> {
		let mut file = syn::parse_file(file)?;
		let mut visitor = RsxFileVisitor::new(self);
		visitor.visit_file_mut(&mut file);
		Ok(visitor.into())
	}
}

pub fn unwrap_macro(expr: &Expr) -> syn::Result<&syn::Macro> {
	if let Expr::Macro(mac) = expr {
		Ok(&mac.mac)
	} else {
		Err(syn::Error::new_spanned(expr, "expected macro"))
	}
}

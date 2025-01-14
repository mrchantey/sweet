use super::RsxFileVisitor;
use super::RsxFileVisitorOut;
use super::RsxPartsTokens;
use super::WalkNodesOutput;
use proc_macro2::TokenStream;
use rstml::node::CustomNode;
use rstml::node::KeyedAttribute;
use rstml::node::NodeElement;
use syn::visit_mut::VisitMut;
use syn::Expr;





pub trait RsxPlugin: Sized {
	/// Called when visiting an rsx [Expr::Macro],
	/// passed as an [Expr] so that the macro can be replaced.
	/// see [macro_or_err] for an easy map.
	fn visit_rsx(&mut self, item: &mut Expr) -> syn::Result<RsxPartsTokens>;

	fn visit_event(
		&mut self,
		item: &KeyedAttribute,
		output: &mut WalkNodesOutput,
	) -> syn::Result<()>;

	/// Opportunity to mutate an element before attributes and children are visited.
	#[allow(unused_variables)]
	fn visit_element<C: CustomNode>(
		&mut self,
		el: &mut NodeElement<C>,
		output: &mut WalkNodesOutput,
	) -> syn::Result<()> {
		Ok(())
	}


	/// specify the identifier that will be parsed
	fn macro_ident() -> &'static str { "rsx" }

	/// Check if a path matches the macro, by default only the last segment is checked
	fn path_matches(path: &syn::Path) -> bool {
		path.segments
			.last()
			.map_or(false, |seg| seg.ident == Self::macro_ident())
	}

	// entrypoint for file (preprosessor) parsing
	fn parse_file(&mut self, file: &str) -> syn::Result<RsxFileVisitorOut> {
		let mut file = syn::parse_file(file)?;
		let mut visitor = RsxFileVisitor::new(self);
		visitor.visit_file_mut(&mut file);
		Ok(visitor.into())
	}

	/// entrypoint for inline (macro) parsing.
	fn parse_tokens(
		&mut self,
		tokens: TokenStream,
	) -> syn::Result<RsxPartsTokens> {
		let mut expr: Expr = syn::parse2(tokens)?;
		self.visit_rsx(&mut expr)
	}
}

pub fn macro_or_err(expr: &Expr) -> syn::Result<&syn::Macro> {
	if let Expr::Macro(mac) = expr {
		Ok(&mac.mac)
	} else {
		Err(syn::Error::new_spanned(expr, "expected macro"))
	}
}


#[cfg(test)]
mod test {

	// struct Foo(usize);

	// impl RsxPlugin for Foo {
	// 	fn visit_rsx(&mut self, _: &mut Expr) -> syn::Result<RsxPartsTokens> {
	// 		let id = self.0;
	// 		self.0 += 1;
	// 		Ok(RsxPartsTokens {
	// 			expr: syn::parse_quote! { #id },
	// 			..Default::default()
	// 		})
	// 	}
	// }

	#[test]
	fn works() {}
}

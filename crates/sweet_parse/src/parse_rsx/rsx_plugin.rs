use super::RsxFileVisitor;
use super::RsxFileVisitorOut;
use super::WalkNodesOutput;
use proc_macro2::TokenStream;
use quote::ToTokens;
use rstml::node::CustomNode;
use rstml::node::KeyedAttribute;
use rstml::node::NodeBlock;
use rstml::node::NodeElement;
use syn::visit_mut::VisitMut;
use syn::Expr;
use syn::File;
use syn::Macro;





pub trait RsxPlugin: Sized {
	// entrypoint for file (preprosessor) parsing
	fn parse_file(
		&mut self,
		file: &str,
	) -> syn::Result<(File, RsxFileVisitorOut)> {
		let mut file = syn::parse_file(file)?;
		let mut visitor = RsxFileVisitor::new(self);
		visitor.visit_file_mut(&mut file);
		Ok((file, visitor.into()))
	}

	/// entrypoint for inline (macro) parsing.
	fn parse_tokens(
		&mut self,
		tokens: TokenStream,
	) -> syn::Result<(TokenStream, WalkNodesOutput)> {
		let mut mac: Macro = syn::parse2(tokens)?;
		let output = self.visit_rsx(&mut mac)?;
		Ok((mac.tokens.to_token_stream(), output))
	}


	/// Called when visiting an rsx macro.
	fn visit_rsx(&mut self, mac: &mut Macro) -> syn::Result<WalkNodesOutput>;

	fn visit_block(&mut self, block: &NodeBlock, output: &mut WalkNodesOutput);
	fn visit_event(
		&mut self,
		item: &KeyedAttribute,
		output: &mut WalkNodesOutput,
	);

	/// Opportunity to view children, useful for text node block encoding
	#[allow(unused_variables)]
	fn visit_element<C: CustomNode>(
		&mut self,
		el: &NodeElement<C>,
		output: &mut WalkNodesOutput,
	);

	/// specify the identifier that will be parsed
	fn macro_ident() -> &'static str { "rsx" }

	/// Check if a path matches the macro, by default only the last segment is checked
	fn path_matches(path: &syn::Path) -> bool {
		path.segments
			.last()
			.map_or(false, |seg| seg.ident == Self::macro_ident())
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

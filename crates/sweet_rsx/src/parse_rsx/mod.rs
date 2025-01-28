mod rsx_node_tokens;
pub mod tokens_to_rstml;
#[allow(unused_imports)]
pub use self::tokens_to_rstml::*;
pub use rsx_node_tokens::*;
pub mod rsx_file_visitor;
#[allow(unused_imports)]
pub use self::rsx_file_visitor::*;
pub mod rstml_to_rsx_tokens;
#[allow(unused_imports)]
pub use self::rstml_to_rsx_tokens::*;
use proc_macro2::TokenStream;
use sweet_core::tokens::RsxRustTokens;
use syn::visit_mut::VisitMut;
use syn::Expr;
use syn::File;


pub struct RsxParser<T> {
	pub include_errors: bool,
	pub macro_ident: String,
	phantom: std::marker::PhantomData<T>,
}

impl<T> Default for RsxParser<T> {
	fn default() -> Self {
		Self {
			include_errors: true,
			macro_ident: "rsx".to_string(),
			phantom: Default::default(),
		}
	}
}

impl<T: RsxRustTokens> RsxParser<T> {
	/// header to add to the top of each rust file
	pub const SHEBANG: &'static str = "// 🍬 AUTOGENERATED BY SWEET 🍬\n// 🍬 AUTOGENERATED BY SWEET 🍬\n// 🍬 AUTOGENERATED BY SWEET 🍬";

	// entrypoint for file (preprosessor) parsing
	pub fn parse_file(
		&mut self,
		file: &str,
	) -> syn::Result<(File, RsxFileVisitorOut<T>)> {
		// errors in preprocessed files are not included, rstml
		// gets confused
		self.include_errors = false;
		let mut file = syn::parse_file(file)?;
		let mut visitor = RsxFileVisitor::new(self);
		visitor.visit_file_mut(&mut file);
		// Validate space
		file.shebang = Some(Self::SHEBANG.to_string());
		Ok((file, visitor.into()))
	}


	/// entrypoint for inline (macro) parsing.
	/// Called when visiting an rsx macro.
	/// Mutated in place for efficient file parsing
	pub fn parse_rsx(&mut self, tokens: &mut TokenStream) -> RstmlToRsx<T> {
		let (nodes, rstml_errors) = tokens_to_rstml(tokens.clone());
		let mut output = RstmlToRsx::default();
		let nodes = output.map_nodes(nodes);

		let RstmlToRsx {
			errors,
			collected_elements,
			..
		} = &output;

		let _ = collected_elements;

		let errors = if self.include_errors {
			let errs = rstml_errors
				.into_iter()
				.map(|e| e.emit_as_expr_tokens())
				.chain(errors.clone());
			quote::quote! {#(#errs;)*}
		} else {
			Default::default()
		};

		*tokens = syn::parse_quote! {{
			#errors
			use sweet::prelude::*;
			#[allow(unused_braces)]
			{
				RsxNode::Fragment(Vec::from([#(#nodes),*]))
			}
		}};
		output
	}

	/// Check if a path matches the macro, by default only the last segment is checked
	pub fn path_matches(&self, path: &syn::Path) -> bool {
		path.segments
			.last()
			.map_or(false, |seg| seg.ident == self.macro_ident)
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
	use sweet_core::prelude::*;
	use sweet_core::rsx::Component;
	use sweet_core::{
		self as sweet,
	};
	use sweet_rsx_macros::rsx;
	use sweet_test::prelude::*;

	#[test]
	fn compiles() {
		let onclick = |_: u32| {};
		let world = "mars";
		let _rsx = rsx! {<div onclick=onclick><p>hello {world}</p></div>};
	}
	#[test]
	fn render_html() {
		let onclick = |_: u32| {};
		let node = rsx! {<div onclick=onclick> the value is {3} </div>};

		expect(RsxToHtml::render(&node))
			.to_be("<div onclick=\"onclick_handler\"> the value is 3</div>");
	}
	#[test]
	fn component_props() {
		struct Child {
			value: usize,
		}
		impl Component for Child {
			fn render(self) -> impl Rsx {
				rsx! {<p>hello {self.value}</p>}
			}
		}
		let node = rsx! {<div> the child is <Child value=38/>! </div>};

		expect(RsxToHtml::render(&node))
			.to_be("<div> the child is <p>hello 38</p>! </div>");
	}
	#[test]
	fn component_children() {
		struct Layout;
		impl Component for Layout {
			fn render(self) -> impl Rsx {
				rsx! {
					<div>
						<h1>welcome</h1>
						<p><slot/></p>
					</div>
				}
			}
		}
		let node = rsx! {<Layout><b>foo</b></Layout>};

		expect(RsxToHtml::render(&node))
			.to_be("<div><h1>welcome</h1><p><b>foo</b></p></div>");
	}
	#[test]
	fn component_slots() {
		struct Layout;
		impl Component for Layout {
			fn render(self) -> impl Rsx {
				rsx! {
					<article>
						<h1>welcome</h1>
						<p><slot name="tagline"/></p>
						<main>
							<slot/>
						</main>
					</article>
				}
			}
		}
		let node = rsx! {
			<Layout>
				<b slot="tagline">what a cool article</b>
				<div>direct child</div>
			</Layout>
		};

		expect(RsxToHtml::render(&node))
			.to_be("<article><h1>welcome</h1><p><b>what a cool article</b></p><main><div>direct child</div></main></article>");
	}
}

use super::*;
use proc_macro2::TokenStream;
use quote::quote;
use quote::ToTokens;
use rstml::node::CustomNode;
use rstml::node::KeyedAttribute;
use rstml::node::Node;
use rstml::node::NodeAttribute;
use rstml::node::NodeBlock;
use rstml::node::NodeElement;
use rstml::node::NodeName;
use sweet_core::prelude::*;

/// The sweet plugin for the rsx! macro.
/// Deliberately no default, any implementers of this trait
/// should decide whether to include errors. For example a macro
/// should but a preprocessor usually should not.
#[derive(Debug, Clone)]
pub struct SweetRsxPlugin {
	/// usually we include errors for macros but not files.
	pub include_errors: bool,
	/// The placeholder for rust blocks in the html, defaults to `§`
	pub placeholder: String,
}

impl SweetRsxPlugin {
	pub fn new_with_errors() -> Self {
		Self {
			include_errors: true,
			placeholder: RsxParts::default_placeholder(),
		}
	}

	pub fn new_no_errors() -> Self {
		Self {
			include_errors: false,
			placeholder: RsxParts::default_placeholder(),
		}
	}
}


impl RsxPlugin for SweetRsxPlugin {
	fn parse_rsx(
		&mut self,
		tokens: &mut TokenStream,
	) -> syn::Result<WalkNodesOutput> {
		let (nodes, rstml_errors) = parse_rstml(tokens.clone());
		let output = WalkNodes::walk_nodes(self, nodes);
		let WalkNodesOutput {
			errors,
			html,
			rust,
			css,
			collected_elements,
		} = output.clone();

		let _ = collected_elements;

		let errors = if self.include_errors {
			let errs = rstml_errors
				.into_iter()
				.map(|e| e.emit_as_expr_tokens())
				.chain(errors);
			quote::quote! {#(#errs;)*}
		} else {
			Default::default()
		};

		*tokens = syn::parse_quote! {{
			#errors
			sweet::prelude::RsxParts {
				rust: vec![#(#rust,)*],
				html: PathOrInline::Inline(#html.to_string()),
				css: PathOrInline::Inline(#css.to_string()),
			}
		}};
		Ok(output)
	}

	fn visit_element<C: CustomNode>(
		&mut self,
		el: &NodeElement<C>,
		output: &mut WalkNodesOutput,
	) {
		let dyn_attrs = el.attributes().iter().any(attribute_is_rusty);

		let dyn_children =
			el.children.iter().any(|c| matches!(c, Node::Block(_)));

		if !dyn_attrs && !dyn_children {
			return;
		}

		// assign s-id key
		output
			.rust
			.push(quote! {sweet::prelude::RsxRust::DynNodeId});
		output.html.push(' ');
		output.html.push_str(&self.placeholder);

		// if dyn_children {
		// 	let encoded = encode_text_block_positions(&el.children);
		// 	output
		// 		.html_string
		// 		.push_str(&format!(" rsx-block-indices=\"{}\"", encoded));
		// }
	}

	fn visit_block(&mut self, block: &NodeBlock, output: &mut WalkNodesOutput) {
		output.rust.push(
			quote! {sweet::prelude::RsxRust::InnerText(#block.to_string())},
		);
		output.html.push(' ');
		output.html.push_str(&self.placeholder);
	}

	/// Sweet plugin allows for event shorthand:
	/// ```ignore
	/// let on_click = |e|{};
	/// rsx!{<div on_click/>}
	/// ```
	fn visit_event(
		&mut self,
		attr: &KeyedAttribute,
		output: &mut WalkNodesOutput,
	) {
		let value = match attr.value() {
			Some(value) => value.clone(),
			// we allow functions with the same name of the event here
			None => {
				// this way we keep the span?
				let key = &attr.key.to_token_stream();
				syn::parse_quote! {#key}
			} // None => syn::parse_str::<syn::Expr>(&attr.key.to_string())?,
		};

		output
			.rust
			.push(quote::quote! {RsxRust::Event(Box::new(#value))});
		output
			.html
			.push_str(&format!(" {}=\"{}\"", attr.key, self.placeholder));
	}
}


/// Attributes are rusty if:
/// - the key is an event (onclick can be shorthand for an ident)
/// - the value is not a "string" or number, meaning its an ident (or true/false but thats ok)
/// - the attribute is a block
fn attribute_is_rusty(a: &NodeAttribute) -> bool {
	match a {
		NodeAttribute::Block(_) => true,
		NodeAttribute::Attribute(attr) => {
			match attr.key {
				NodeName::Block(_) => return true,
				_ => {
					if attr.key.to_string().starts_with("on") {
						return true;
					}
				}
			}
			match attr.value() {
				None => false,
				// only a number or string is not 'rust'
				Some(syn::Expr::Lit(_)) => false,
				Some(_) => true,
			}
		}
	}
}

#[cfg(test)]
mod test {
	use sweet::prelude::*;

	#[test]
	fn events() {
		let mut plugin = SweetRsxPlugin::new_no_errors();
		// macro with an event and block
		// raw text nodes are trimmed
		let mut tokens = quote::quote! {
			<div onclick></div>
		};

		let out = plugin.parse_rsx(&mut tokens).unwrap();
		let tokens_str = tokens.to_string();
		// let tokens_str = prettyplease::unparse(
		// 	&syn::parse_file(&tokens.to_string()).unwrap(),
		// );
		// println!("{}", tokens_str);

		expect(&tokens_str).to_contain("(onclick)");
		expect(&tokens_str).not().to_start_with("rsx!");
		expect(&out.html).to_be(r#"<div rsx-id="0" onclick="§"></div>"#);
	}
	#[test]
	fn text_blocks() {
		let mut plugin = SweetRsxPlugin::new_no_errors();
		// raw text nodes are trimmed
		let mut tokens = quote::quote! {
			<div>"the "{value}"th "<bold>value</bold> is {value}</div>
		};
		let out = plugin.parse_rsx(&mut tokens).unwrap();
		expect(out.html)
			.to_be(r#"<div rsx-id="0">the  §th <bold>value</bold>is §</div>"#);
	}
	#[test]
	#[ignore]
	fn child_component() {
		let mut plugin = SweetRsxPlugin::new_no_errors();
		// raw text nodes are trimmed
		let mut tokens = quote::quote! {
			rsx!{<body><Header>"sweet "<b/>as</b></Header></body>}
		};
		let out = plugin.parse_rsx(&mut tokens).unwrap();
		expect(out.html).to_be(r#"<body>§§sweet <b>as</>§§</body>"#);
	}
}

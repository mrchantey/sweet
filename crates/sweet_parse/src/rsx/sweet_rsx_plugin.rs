use super::*;
use quote::ToTokens;
use rstml::node::CustomNode;
use rstml::node::KeyedAttribute;
use rstml::node::Node;
use rstml::node::NodeAttribute;
use rstml::node::NodeBlock;
use rstml::node::NodeElement;
use rstml::node::NodeName;
use syn::spanned::Spanned;


/// The sweet plugin for the rsx! macro.
#[derive(Debug, Clone)]
pub struct SweetRsxPlugin {
	/// every element that directly contains rust code is assigned an incremental id
	pub rsx_id_incr: usize,
	/// The placeholder for rust blocks in the html, defaults to `§`
	pub placeholder: String,
}

impl Default for SweetRsxPlugin {
	fn default() -> Self {
		Self {
			rsx_id_incr: 0,
			placeholder: "§".to_string(),
		}
	}
}


impl RsxPlugin for SweetRsxPlugin {
	fn visit_rsx(
		&mut self,
		expr: &mut syn::Expr,
	) -> syn::Result<WalkNodesOutput> {
		let mac = macro_or_err(expr)?;
		let (nodes, rstml_errors) = parse_rstml(mac.tokens.clone());
		let output = WalkNodes::walk_nodes(self, nodes);
		let WalkNodesOutput {
			errors,
			html,
			rust,
			css,
			collected_elements,
		} = output.clone();

		let _ = collected_elements;

		let errors = rstml_errors
			.into_iter()
			.map(|e| e.emit_as_expr_tokens())
			.chain(errors);

		let span = mac.tokens.span();
		*expr = syn::parse_quote_spanned! {span=> {
			#(#errors;)*
			sweet::prelude::RsxParts {
				rust: vec![#(#rust,)*],
				html: PathOrInline::Inline(#html),
				css: PathOrInline::Inline(#css),
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
		// the value assigned at the component builder step
		let id = self.rsx_id_incr;
		self.rsx_id_incr += 1;
		output.html.push_str(&format!(" rsx-id=\"{}\"", id));

		// if dyn_children {
		// 	let encoded = encode_text_block_positions(&el.children);
		// 	output
		// 		.html_string
		// 		.push_str(&format!(" rsx-block-indices=\"{}\"", encoded));
		// }
	}

	fn visit_block(&mut self, block: &NodeBlock, output: &mut WalkNodesOutput) {
		output.rust.push(block.to_token_stream());
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

		let boxed = quote::quote! {Box::new(#value)};

		output.rust.push(boxed);
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
	// use crate::prelude::*;
	use super::RsxPlugin;
	use super::*;
	use quote::ToTokens;
	use sweet::prelude::*;


	#[test]
	fn works() {
		let mut plugin = SweetRsxPlugin::default();
		// macro with an event and block
		// raw text nodes are trimmed
		let mac = quote::quote! {
			rsx!{<div onclick>"the "{value}"th "<bold>value</bold> is {value}</div>}
		};

		let (expr, out) = plugin.parse_tokens(mac).unwrap();

		expect(expr.to_token_stream().to_string()).to_contain("(onclick)");
		expect(out.html).to_be(r#"<div rsx-id="0" onclick="§">the  §th <bold>value</bold>is §</div>"#);
		// println!("{}", expr.to_token_stream().to_string());
		// println!("{}", out.html);
	}
}

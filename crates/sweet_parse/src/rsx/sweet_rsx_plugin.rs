use super::*;
use quote::ToTokens;
use rstml::node::CustomNode;
use rstml::node::KeyedAttribute;
use rstml::node::KeyedAttributeValue;
use rstml::node::NodeAttribute;
use rstml::node::NodeElement;

#[derive(Default)]
pub struct SweetRsxPlugin {
	/// number of html elements containing
	/// rust code
	pub num_elements_with_blocks: usize,
	pub num_rsx_macros: usize,
}

impl RsxPlugin for SweetRsxPlugin {
	fn visit_rsx(
		&mut self,
		expr: &mut syn::Expr,
	) -> syn::Result<RsxPartsTokens> {
		let mac = macro_or_err(expr)?;
		let parts = RsxPartsTokens::parse(self, mac.tokens.clone());
		*expr = parts.expr.clone();
		// *expr = syn::parse_quote! {"howdy"};
		Ok(parts)
	}

	fn visit_element<C: CustomNode>(
		&mut self,
		el: &mut NodeElement<C>,
		_output: &mut WalkNodesOutput,
	) -> syn::Result<()> {
		let dyn_attrs = el.attributes().iter().any(event_or_block_attribute);

		let dyn_children = el
			.children
			.iter()
			.any(|c| matches!(c, rstml::node::Node::Block(_)));

		if !dyn_attrs && !dyn_children {
			return Ok(());
		}

		// assign s-id key
		// the value assigned at the component builder step
		el.attributes_mut()
			.push(NodeAttribute::Attribute(KeyedAttribute {
				key: syn::parse_quote!(s - id),
				possible_value: KeyedAttributeValue::None,
			}));
		Ok(())
	}

	/// Sweet plugin allows for event shorthand:
	/// ```
	/// let on_click = |e|{}
	/// rsx!{<div on_click/>}
	/// ```
	fn visit_event(
		&mut self,
		attr: &KeyedAttribute,
		output: &mut WalkNodesOutput,
	) -> syn::Result<()> {
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

		let index = output.rust_events.len();
		output.rust_events.push(boxed);
		// leading space
		output.html_string.push_str(&format!(
			" {}=\"_sweet.event({},event)\"",
			attr.key, index
		));

		Ok(())
	}
}

// fn string_to_node_name(str: &str) -> NodeName{

// }


fn event_or_block_attribute(a: &NodeAttribute) -> bool { true }

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
		let value = 1;
		let mac = quote::quote! {
			rsx!{<div onclick>the {value}th value is {value}</div>}
		};

		let out = plugin.parse_tokens(mac).unwrap();

		expect(out.expr.to_token_stream().to_string()).to_contain("(onclick)");
		println!("{}", out.expr.to_token_stream().to_string());
		println!("{}", out.html);
	}
}

use crate::prelude::*;


/// A simple rsx implementation that
/// calls to_string on all rsx parts
#[derive(Debug)]
pub struct StringRsx;

impl RsxRust for StringRsx {
	type NodeBlock = String;
	type AttributeBlock = String;
	type AttributeBlockValue = String;
	fn attribute_block_to_string(block: &Self::AttributeBlock) -> String {
		block.clone()
	}
	fn attribute_block_value_to_string(
		block: &Self::AttributeBlockValue,
	) -> String {
		block.clone()
	}
	fn block_to_string(block: &Self::NodeBlock) -> String { block.clone() }
}


pub type RsxNodes = crate::prelude::RsxTree<StringRsx>;
pub type Children = Vec<RsxNode<StringRsx>>;

pub trait Component {
	fn render(self) -> RsxNodes;
}

impl<T: Component> Rsx for T {
	type Node = StringRsx;
	fn into_rsx_tree(self) -> crate::prelude::RsxTree<Self::Node> {
		self.render()
	}
}


#[cfg(feature = "tokens")]
use proc_macro2::TokenStream;
#[cfg(feature = "tokens")]
use quote::quote;

#[cfg(feature = "tokens")]
impl RsxRustTokens for StringRsx {
	fn ident() -> TokenStream {
		quote! {sweet::string_rsx::StringRsx}
	}

	fn map_block(block: &TokenStream) -> TokenStream {
		quote! { RsxNode::TextBlock{
				initial: #block.to_string(),
				register_effect: Box::new(|| {}),
			 }
		}
	}

	fn map_attribute_block(block: &TokenStream) -> TokenStream {
		quote! { #block.to_string() }
	}

	fn map_attribute_value(key: &str, value: &TokenStream) -> TokenStream {
		if key.starts_with("on") {
			let str = format!("{key}_handler");
			quote! { #str.to_string() }
		} else {
			quote! { #value.to_string() }
		}
	}
}

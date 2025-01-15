#![cfg(feature = "quote")]
use super::*;
use proc_macro2::TokenStream;
use quote::quote;
use quote::ToTokens;

impl ToTokens for HtmlPartial {
	fn to_tokens(&self, tokens: &mut TokenStream) {
		let nodes = &self.nodes;
		quote! {
			HtmlPartial {
				nodes: vec![#(#nodes),*],
			}
		}
		.to_tokens(tokens);
	}
}

impl ToTokens for Element {
	fn to_tokens(&self, tokens: &mut TokenStream) {
		let tag = &self.tag;
		let attributes = &self.attributes;
		let children = &self.children;
		let self_closing = self.self_closing;
		quote! {
			Element {
				tag: #tag.to_string(),
				attributes: vec![#(#attributes),*],
				children: vec![#(#children),*],
				self_closing: #self_closing,
			}
		}
		.to_tokens(tokens);
	}
}

impl ToTokens for Node {
	fn to_tokens(&self, tokens: &mut TokenStream) {
		match self {
			Node::Doctype => quote!(Node::Doctype),
			Node::Comment(s) => quote!(Node::Comment(#s.to_string())),
			Node::Element(e) => quote!(Node::Element(#e)),
			Node::Text(s) => quote!(Node::Text(#s.to_string())),
			Node::TextBlock => quote!(Node::TextBlock),
			Node::Component(c) => quote!(Node::Component(Vec::from([#(#c,)*]))),
		}
		.to_tokens(tokens);
	}
}

impl ToTokens for Attribute {
	fn to_tokens(&self, tokens: &mut TokenStream) {
		match self {
            Attribute::Key { key } => quote!(Attribute::Key { key: #key.to_string() }),
            Attribute::KeyValue { key, value } => {
                quote!(Attribute::KeyValue { key: #key.to_string(), value: #value.to_string() })
            }
            Attribute::BlockValue { key } => {
                quote!(Attribute::BlockValue { key: #key.to_string() })
            }
            Attribute::Block => quote!(Attribute::Block),
        }.to_tokens(tokens);
	}
}

#![cfg(feature = "quote")]
use super::*;
use proc_macro2::TokenStream;
use quote::quote;
use quote::ToTokens;

impl<R: ToTokens> ToTokens for RsxTree<R> {
	fn to_tokens(&self, tokens: &mut TokenStream) {
		let nodes = &self.nodes;
		quote! {
			RsxTree {
				nodes: vec![#(#nodes),*],
			}
		}
		.to_tokens(tokens);
	}
}

impl<R: ToTokens> ToTokens for Element<R> {
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

impl<R: ToTokens> ToTokens for Node<R> {
	fn to_tokens(&self, tokens: &mut TokenStream) {
		match self {
			Node::Doctype => quote!(Node::Doctype),
			Node::Comment(s) => quote!(Node::Comment(#s.to_string())),
			Node::Element(e) => quote!(Node::Element(#e)),
			Node::Text(s) => quote!(Node::Text(#s.to_string())),
			Node::TextBlock(r) => quote!(Node::TextBlock(#r)),
			Node::Component(r, c) => {
				quote!(Node::Component(#r,Vec::from([#(#c,)*])))
			}
		}
		.to_tokens(tokens);
	}
}

impl<R: ToTokens> ToTokens for Attribute<R> {
	fn to_tokens(&self, tokens: &mut TokenStream) {
		match self {
            Attribute::Key { key } => quote!(Attribute::Key { key: #key.to_string() }),
            Attribute::KeyValue { key, value } => {
                quote!(Attribute::KeyValue { key: #key.to_string(), value: #value.to_string() })
            }
            Attribute::BlockValue { key,value } => {
                quote!(Attribute::BlockValue { key: #key.to_string(),value: #value })
            }
            Attribute::Block(r) => quote!(Attribute::Block(#r)),
        }.to_tokens(tokens);
	}
}

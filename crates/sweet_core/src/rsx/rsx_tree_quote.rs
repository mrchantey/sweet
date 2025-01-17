use super::RsxAttribute;
use super::RsxElement;
use super::RsxNode;
use super::RsxTree;
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

impl<R: ToTokens> ToTokens for RsxElement<R> {
	fn to_tokens(&self, tokens: &mut TokenStream) {
		let tag = &self.tag;
		let attributes = &self.attributes;
		let children = &self.children;
		let self_closing = self.self_closing;
		quote! {
			RsxElement {
				tag: #tag.to_string(),
				attributes: vec![#(#attributes),*],
				children: vec![#(#children),*],
				self_closing: #self_closing,
			}
		}
		.to_tokens(tokens);
	}
}

impl<R: ToTokens> ToTokens for RsxNode<R> {
	fn to_tokens(&self, tokens: &mut TokenStream) {
		match self {
			RsxNode::Doctype => quote!(RsxNode::Doctype),
			RsxNode::Comment(s) => quote!(RsxNode::Comment(#s.to_string())),
			RsxNode::Element(e) => quote!(RsxNode::Element(#e)),
			RsxNode::Text(s) => quote!(RsxNode::Text(#s.to_string())),
			RsxNode::TextBlock(r) => quote!(RsxNode::TextBlock(#r)),
			RsxNode::Component(r, c) => {
				quote!(RsxNode::Component(#r,Vec::from([#(#c,)*])))
			}
		}
		.to_tokens(tokens);
	}
}

impl<R: ToTokens> ToTokens for RsxAttribute<R> {
	fn to_tokens(&self, tokens: &mut TokenStream) {
		match self {
            RsxAttribute::Key { key } => quote!(RsxAttribute::Key { key: #key.to_string() }),
            RsxAttribute::KeyValue { key, value } => {
                quote!(RsxAttribute::KeyValue { key: #key.to_string(), value: #value.to_string() })
            }
            RsxAttribute::BlockValue { key,value } => {
                quote!(RsxAttribute::BlockValue { key: #key.to_string(),value: #value })
            }
            RsxAttribute::Block(r) => quote!(RsxAttribute::Block(#r)),
        }.to_tokens(tokens);
	}
}

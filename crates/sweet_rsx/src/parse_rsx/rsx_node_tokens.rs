use proc_macro2::TokenStream;
use quote::quote;
use quote::ToTokens;
use sweet_core::tokens::RsxRustTokens;

pub enum RsxNodeTokens<T> {
	Phantom(std::marker::PhantomData<T>),
	Doctype,
	Text(String),
	Comment(String),
	Block(TokenStream),
	Element {
		tag: String,
		attributes: Vec<RsxAttributeTokens<T>>,
		children: Vec<RsxNodeTokens<T>>,
		self_closing: bool,
	},
	Fragment(Vec<RsxNodeTokens<T>>),
	Component(TokenStream),
}

impl<T: RsxRustTokens> ToTokens for RsxNodeTokens<T> {
	fn to_tokens(&self, tokens: &mut TokenStream) {
		match self {
			RsxNodeTokens::Phantom(_) => unreachable!(),
			RsxNodeTokens::Doctype => quote!(RsxNode::Doctype),
			RsxNodeTokens::Text(text) => {
				quote!(RsxNode::Text(#text.to_string()))
			}
			RsxNodeTokens::Comment(comment) => {
				quote!(RsxNode::Comment(#comment.to_string()))
			}
			RsxNodeTokens::Block(block) => {
				let block = T::map_block(block);
				quote!(RsxNode::Block(#block))
			}
			RsxNodeTokens::Element {
				tag,
				attributes,
				children,
				self_closing,
			} => {
				let children = children_to_tokens(children);
				quote!(RsxNode::Element(RsxElement {
					tag: #tag.to_string(),
					attributes: vec![#(#attributes),*],
					children: #children,
					self_closing: #self_closing,
				}))
			}
			RsxNodeTokens::Fragment(vec) => {
				quote!(#(#vec),*)
			}
			RsxNodeTokens::Component(token_stream) => quote!(#token_stream),
		}
		.to_tokens(tokens);
	}
}

/// Map children to tokens,
/// flattening fragments and components
fn children_to_tokens<T: RsxRustTokens>(
	children: &Vec<RsxNodeTokens<T>>,
) -> TokenStream {
	let add = children.into_iter().map(|child| match child {
		RsxNodeTokens::Phantom(_) => unreachable!(),
		RsxNodeTokens::Fragment(children) => {
			let children = children_to_tokens(children);
			quote!(vec.extend(#children);)
		}
		RsxNodeTokens::Component(component) => quote!(vec.extend(#component)),
		RsxNodeTokens::Block(block) => {
			let block = T::map_block(block);
			// unimplemented!("this may be one or many?")
			quote!(vec.push(RsxNode::Block(#block)))
		}
		_ => quote!(vec.push(#child)),
	});

	quote!({
		let mut vec = Vec::new();
		#(#add;)*
		vec
	})
}

pub enum RsxAttributeTokens<T> {
	Phantom(std::marker::PhantomData<T>),
	Key { key: String },
	KeyValue { key: String, value: String },
	BlockValue { key: String, value: TokenStream },
	Block(TokenStream),
}

impl<T: RsxRustTokens> ToTokens for RsxAttributeTokens<T> {
	fn to_tokens(&self, tokens: &mut TokenStream) {
		match self {
			RsxAttributeTokens::Phantom(_) => unreachable!(),
			RsxAttributeTokens::Key{key} => {
				quote!(RsxAttribute::Key { key: #key.to_string() })
			}
			RsxAttributeTokens::KeyValue { key, value } => {
				quote!(RsxAttribute::KeyValue { key: #key.to_string(), value: #value.to_string() })
		}
		RsxAttributeTokens::BlockValue { key,value } => {
			let value = T::map_attribute_value(key, value);
			quote!(RsxAttribute::BlockValue { key: #key.to_string(),value: #value })
		}
		RsxAttributeTokens::Block(block) => {
			let block = T::map_attribute_block(block);
			quote!(RsxAttribute::Block(#block))
		}
	}
		.to_tokens(tokens);
	}
}

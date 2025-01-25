use proc_macro2::TokenStream;
use quote::quote;
use quote::ToTokens;

pub enum RsxNodeTokens {
	Doctype,
	Text(String),
	Comment(String),
	Block(TokenStream),
	Element {
		tag: String,
		attributes: Vec<RsxAttributeTokens>,
		children: Vec<RsxNodeTokens>,
		self_closing: bool,
	},
	Fragment(Vec<RsxNodeTokens>),
	Component(TokenStream),
}

// impl Node for RsxNodeTokens {
// 	fn children(&self) -> Option<&Vec<Self>> {
// 		match self {
// 			RsxNodeTokens::Element { children, .. } => Some(children),
// 			RsxNodeTokens::Fragment(vec) => Some(vec),
// 			_ => None,
// 		}
// 	}
// 	fn children_mut(&mut self) -> Option<&mut Vec<Self>> {
// 		match self {
// 			RsxNodeTokens::Element { children, .. } => Some(children),
// 			RsxNodeTokens::Fragment(vec) => Some(vec),
// 			_ => None,
// 		}
// 	}
// 	fn take_children(&mut self) -> Option<Vec<Self>> {
// 		match self {
// 			RsxNodeTokens::Element { children, .. } => {
// 				Some(std::mem::take(children))
// 			}
// 			RsxNodeTokens::Fragment(vec) => Some(std::mem::take(vec)),
// 			_ => None,
// 		}
// 	}
// }

pub enum RsxAttributeTokens {
	Key { key: String },
	KeyValue { key: String, value: String },
	BlockValue { key: String, value: TokenStream },
	Block(TokenStream),
}

impl ToTokens for RsxNodeTokens {
	fn to_tokens(&self, tokens: &mut TokenStream) {
		match self {
			RsxNodeTokens::Doctype => quote!(RsxNode::Doctype),
			RsxNodeTokens::Text(text) => {
				quote!(RsxNode::Text(#text.to_string()))
			}
			RsxNodeTokens::Comment(comment) => {
				quote!(RsxNode::Comment(#comment.to_string()))
			}
			RsxNodeTokens::Block(block) => {
				quote!(RsxNode::Block(#block))
			}
			RsxNodeTokens::Element {
				tag,
				attributes,
				children,
				self_closing,
			} => {
				let children = flatten_children(children);
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


fn flatten_children(children: &Vec<RsxNodeTokens>) -> TokenStream {
	let add = children.into_iter().map(|child| match child {
		RsxNodeTokens::Fragment(children) => {
			let children = flatten_children(children);
			quote!(vec.extend(#children);)
		}
		RsxNodeTokens::Component(component) => quote!(vec.extend(#component)),
		RsxNodeTokens::Block(block) => {
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

impl ToTokens for RsxAttributeTokens {
	fn to_tokens(&self, tokens: &mut TokenStream) {
		match self {
		RsxAttributeTokens::Key{key} => {
				quote!(RsxAttribute::Key { key: #key.to_string() })
			}
		RsxAttributeTokens::KeyValue { key, value } => {
				quote!(RsxAttribute::KeyValue { key: #key.to_string(), value: #value.to_string() })
		}
		RsxAttributeTokens::BlockValue { key,value } => {
				quote!(RsxAttribute::BlockValue { key: #key.to_string(),value: #value })
		}
		RsxAttributeTokens::Block(r) => quote!(RsxAttribute::Block(#r)),		}
		.to_tokens(tokens);
	}
}


// pub struct ParseStringNodeTokens {
// 	position: TreePosition,
// }

// impl TreeVisitorMut<RsxNodeTokens> for ParseStringNodeTokens {
// 	fn visit_node(
// 		&mut self,
// 		node: &mut RsxNodeTokens,
// 	) -> sweet_core::prelude::ParseResult<()> {
// 		match node {
// 			RsxNodeTokens::Block(token_stream) => {
// 				*token_stream = quote! {#token_stream.to_string()}
// 			}
// 			RsxNodeTokens::Element { attributes, .. } => {
// 				for attribute in attributes {
// 					match attribute {
// 						RsxAttributeTokens::BlockValue { value, .. } => {
// 							*value = quote! {#value.to_string()}
// 						}
// 						RsxAttributeTokens::Block(token_stream) => {
// 							*token_stream = quote! {#token_stream.to_string()}
// 						}
// 						_ => {}
// 					}
// 				}
// 			}
// 			_ => {}
// 		}

// 		Ok(())
// 	}

// 	fn leave_node(
// 		&mut self,
// 		node: &mut RsxNodeTokens,
// 	) -> sweet_core::prelude::ParseResult<()> {
// 		Ok(())
// 	}

// 	fn visit_children(
// 		&mut self,
// 		children: &mut Vec<RsxNodeTokens>,
// 	) -> sweet_core::prelude::ParseResult<()> {
// 		Ok(())
// 	}

// 	fn leave_children(
// 		&mut self,
// 		children: &mut Vec<RsxNodeTokens>,
// 	) -> sweet_core::prelude::ParseResult<()> {
// 		Ok(())
// 	}
// }



// fn visit_attribute(&mut self, attr: NodeAttribute) -> RsxAttributeTokens {
// 	match attr {
// 		NodeAttribute::Block(block) => RsxAttributeTokens::Block(
// 			quote! {RustParts::AttributeBlock(#block)},
// 		),
// 		NodeAttribute::Attribute(attr) => {
// 			let key = attr.key.to_string();

// 			if key.starts_with("on") {
// 				let tokens = if let Some(value) = attr.value() {
// 					value.to_token_stream()
// 				} else {
// 					// default to a function called onclick
// 					Ident::new(&key, key.span()).to_token_stream()
// 				};
// 				RsxAttribute::BlockValue {
// 					key,
// 					value: quote! {RustParts::Event(Box::new(#tokens))},
// 				}
// 			} else if let Some(value) = attr.value() {
// 				match value {
// 					// only literals (string, number, bool) are not rusty
// 					syn::Expr::Lit(expr_lit) => {
// 						let value = match &expr_lit.lit {
// 							syn::Lit::Str(s) => s.value(),
// 							other => other.to_token_stream().to_string(),
// 						};
// 						RsxAttribute::KeyValue { key, value }
// 					}
// 					tokens => RsxAttribute::BlockValue {
// 						key,
// 						value: quote! {RustParts::AttributeValue(#tokens.to_string())},
// 					},
// 				}
// 			} else {
// 				RsxAttribute::Key { key }
// 			}
// 		}
// 	}

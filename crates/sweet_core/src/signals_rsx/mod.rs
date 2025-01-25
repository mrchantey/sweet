mod signal;
// use crate::rsx::RsxAttribute;
// use crate::rsx::RsxNode;
// use crate::rsx::RsxRust;
use crate::prelude::*;
pub use signal::*;

pub struct SignalsRsx;



impl SignalsRsx {
	pub fn register_node_block<M>(
		block: impl 'static + Clone + IntoRsx<M>,
	) -> Box<dyn FnOnce()> {
		Box::new(move || {
			effect(move || {
				let node = block.clone().into_rsx();
				println!("would update node: {}", node.build_string());
				// todo!();
			});
		})
	}
	pub fn register_attribute_block(
		mut block: impl 'static + FnMut() -> RsxAttribute,
	) -> Box<dyn FnOnce()> {
		Box::new(move || {
			effect(move || {
				let attrs = block();
				println!("would update attributes: {}", attrs.build_string());
				todo!();
			});
		})
	}
	pub fn register_attribute_block_value<T: ToString>(
		key: &str,
		mut block: impl 'static + FnMut() -> T,
	) -> Box<dyn FnOnce()> {
		let key = key.to_string();
		Box::new(move || {
			effect(move || {
				let value = block().to_string();
				println!("would update attribute {key}: {value}");
				todo!();
			});
		})
	}
}

// yes it looks right but i think these types will always be the same
#[cfg(feature = "tokens")]
use proc_macro2::TokenStream;
#[cfg(feature = "tokens")]
use quote::quote;

#[cfg(feature = "tokens")]
impl RsxRustTokens for SignalsRsx {
	fn ident() -> TokenStream {
		quote! {sweet::signals_rsx::SignalsRsx}
	}

	fn map_node_block(block: &TokenStream) -> TokenStream {
		let ident = Self::ident();
		quote! {
			{
				let block = #block;
				RsxNode::TextBlock{
					register_effect: #ident::register_node_block(block.clone()),
					initial: block.into_rsx().build_string(),
				}
			}
		}
	}

	fn map_attribute_block(block: &TokenStream) -> TokenStream {
		let ident = Self::ident();
		quote! { RsxAttribute::Block{
				initial: #block
				register_effect: #ident::register_attribute_block(#block),
			}
		}
	}

	fn map_attribute_value(key: &str, value: &TokenStream) -> TokenStream {
		if key.starts_with("on") {
			// events unsupported for string_rsx
			let str = format!("{key}_handler");
			quote! { RsxAttribute::KeyValue{
					key: #key.to_string(),
					value: #str.to_string()
				}
			}
		} else {
			let ident = Self::ident();
			quote! { RsxAttributeBlockValue{
					key: #key.to_string(),
					initial: #value.to_string(),
					register_effect: #ident::register_attribute_block_value(#key, #value),
				}
			}
		}
	}
}

#[cfg(test)]
mod test {
	use super::signal;
	use crate::prelude::*;
	// use sweet::prelude::*;
	use sweet_rsx_macros::rsx;


	#[test]
	fn works() {
		let (get, set) = signal(7);

		let rsx = rsx! {<div>{get}</div>};
		rsx.register_effects();
		set(8);
		set(9);
	}
}

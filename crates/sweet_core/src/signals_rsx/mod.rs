mod signal;
// use crate::rsx::RsxAttribute;
// use crate::rsx::RsxNode;
// use crate::rsx::RsxRust;
use crate::prelude::*;
pub use signal::*;

pub struct SignalsRsx;



impl SignalsRsx {
	pub fn map_node_block<M>(
		block: impl 'static + Clone + IntoRsx<M>,
	) -> RsxNode {
		RsxNode::Block {
			initial: Box::new(block.clone().into_rsx()),
			register_effect: Box::new(move |cx| {
				let cx = cx.clone();
				effect(move || {
					let block = block.clone();
					let cx = cx.clone();
					Hydrate::with(move |hydrated| {
						let node = block.clone().into_rsx();
						// println!(
						// 	"would update node for {:?}\n{}:{}",
						// 	cx,
						// 	node.as_ref(),
						// 	node.render()
						// );
						hydrated.update_rsx_node(node, &cx).unwrap();
					});
					// todo!();
				});
			}),
		}
	}
	pub fn map_attribute_block(
		mut block: impl 'static + FnMut() -> RsxAttribute,
	) -> RsxAttribute {
		RsxAttribute::Block {
			initial: vec![block()],
			register_effect: Box::new(move |cx| {
				let cx = cx.clone();
				effect(move || {
					let attrs = block();
					println!(
						"would update attributes for {cx}\n{}",
						attrs.render()
					);
					todo!();
				});
			}),
		}
	}
	pub fn map_attribute_value<M>(
		key: &str,
		block: impl 'static + Clone + IntoRsxAttributeValue<M>,
	) -> RsxAttribute {
		let key = key.to_string();
		RsxAttribute::BlockValue {
			key: key.clone(),
			initial: block.clone().into_attribute_value(),
			register_effect: Box::new(move |cx| {
				let cx = cx.clone();
				effect(move || {
					let value = block.clone().into_attribute_value();
					println!("would update attribute for {cx}\n{key}: {value}");
					todo!();
				});
			}),
		}
	}
	pub fn map_event<T: ToString>(
		key: &str,
		// todo event types
		_block: impl 'static + Clone + FnMut(T),
	) -> RsxAttribute {
		let key = key.to_string();
		let str = format!("{key}_handler");
		RsxAttribute::KeyValue { key, value: str }
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

		let rsx = || rsx! {<div>value is {get}</div>};
		Hydrate::set(HydratedHtml::new(
			rsx.clone()().into_html(&mut RenderOptions::resumable()),
			HtmlConstants::default(),
		));

		rsx().register_effects();
		expect(&Hydrate::with(|h| h.render()))
			.to_be("<div data-sweet-id=\"0\" data-sweet-blocks=\"0-9-1\">value is 7</div>");
		set(8);
		expect(&Hydrate::with(|h| h.render()))
			.to_be("<div data-sweet-id=\"0\" data-sweet-blocks=\"0-9-1\">value is 8</div>");
		set(9);
		expect(&Hydrate::with(|h| h.render()))
			.to_be("<div data-sweet-id=\"0\" data-sweet-blocks=\"0-9-1\">value is 9</div>");
	}
}

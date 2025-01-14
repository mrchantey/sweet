use super::*;


#[derive(Default)]
pub struct SweetRsxPlugin {
	/// number of html elements containing
	/// rust code
	pub num_elements_with_blocks: usize,
	pub num_rsx_macros: usize,
}

impl RsxPlugin for SweetRsxPlugin {
	fn parse_rsx(
		&mut self,
		expr: &mut syn::Expr,
	) -> syn::Result<RsxPartsTokens> {
		let mac = unwrap_macro(expr)?;
		let parts = RsxPartsTokens::parse(self, mac.tokens.clone());
		*expr = parts.expr.clone();
		*expr = syn::parse_quote! {"howdy"};
		Ok(parts)
	}
}
// fn parse_item(&mut self, item: &mut Item) -> syn::Result<()> {
// 	if let Item::Impl(impl_block) = item {
// 		let Some(into_rsx_func) = try_get_into_rsx(impl_block) else {
// 			return Ok(());
// 		};
// 		*item = parse_into_rsx_func(plugin, into_rsx_func)?
// 	}
// 	Ok(())
// }

// fn parse_file(&mut self, file: &mut File) -> syn::Result<()> {
// 	for item in &mut file.items {
// 		self.parse_item(item)?;
// 	}
// 	Ok(())
// }


// fn parse_into_rsx_func(
// 	plugin: &impl RsxPlugin,
// 	into_rsx_func: &ImplItemFn,
// ) -> syn::Result<Item> {
// 	let mut block = into_rsx_func.block.clone();
// 	RsxVisitor::file_rsx_to_parts(plugin, &mut block);
// 	let rust = hydrate_impl.iter().map(|output| &output.rust);
// 	let ident = &into_rsx_func.sig.ident;

// 	let hydrate_client_impl = syn::parse_quote! {
// 		impl sweet::prelude::HydrateClient for #ident {
// 			fn hydrate(&self) {
// 				#(#rust)*
// 			}
// 		}
// 	};

// 	Ok(hydrate_client_impl)
// }


// /// Try to extract the `into_rsx` function from an `IntoRsx` impl block
// fn try_get_into_rsx(impl_block: &ItemImpl) -> Option<&ImplItemFn> {
// 	if !is_into_rsx_impl(impl_block) {
// 		return None;
// 	}
// 	for impl_item in &impl_block.items {
// 		if let ImplItem::Fn(method) = impl_item {
// 			if method.sig.ident == "into_rsx" {
// 				return Some(method);
// 			}
// 		}
// 	}
// 	None
// }

// /// Check if the impl is an implementation of `IntoRsx`
// fn is_into_rsx_impl(impl_block: &ItemImpl) -> bool {
// 	match impl_block.trait_.as_ref() {
// 		Some((_, path, _)) => path
// 			.segments
// 			.last()
// 			.map(|segment| segment.ident == "IntoRsx")
// 			.unwrap_or(false),
// 		None => false,
// 	}
// }


#[cfg(test)]
mod test {
	// use crate::prelude::*;
	// use sweet::prelude::*;

	#[test]
	fn works() {

		// expect(true).to_be_false();
	}
}

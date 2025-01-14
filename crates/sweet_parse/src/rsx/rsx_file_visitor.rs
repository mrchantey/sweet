use crate::prelude::*;
use syn::visit_mut;
use syn::visit_mut::VisitMut;
use syn::Expr;
use syn::ExprMacro;



/// The rsx visitor is used by file (preprocessor) parsers.
pub struct RsxFileVisitor<'a, P> {
	plugin: &'a mut P,
	/// The rsx macros found in the function
	macros: Vec<RsxPartsTokens>,
	/// Errors that occurred while parsing the rsx macro
	errors: Vec<syn::Error>,
}

pub struct RsxFileVisitorOut {
	pub macros: Vec<RsxPartsTokens>,
	pub errors: Vec<syn::Error>,
}

impl<'a, P> Into<RsxFileVisitorOut> for RsxFileVisitor<'a, P> {
	fn into(self) -> RsxFileVisitorOut {
		RsxFileVisitorOut {
			macros: self.macros,
			errors: self.errors,
		}
	}
}

impl<'a, P: RsxPlugin> RsxFileVisitor<'a, P> {
	pub fn new(plugin: &'a mut P) -> Self {
		Self {
			plugin,
			macros: Vec::new(),
			errors: Vec::new(),
		}
	}
}

impl<'a, P: RsxPlugin> VisitMut for RsxFileVisitor<'a, P> {
	fn visit_expr_mut(&mut self, expr: &mut Expr) {
		// First visit any child nodes
		visit_mut::visit_expr_mut(self, expr);

		if let Expr::Macro(ExprMacro { mac, .. }) = expr {
			if P::path_matches(&mac.path) {
				match self.plugin.visit_rsx(expr) {
					Ok(parts) => self.macros.push(parts),
					Err(e) => self.errors.push(e),
				}
			}
		}
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

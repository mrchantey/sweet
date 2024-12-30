use proc_macro2::TokenStream;
use quote::quote;
use quote::ToTokens;
use syn::ItemFn;
use syn::ReturnType;
use syn::Type;

pub struct SweetTestAttr;

impl SweetTestAttr {
	pub fn parse(
		_attr: proc_macro::TokenStream,
		input: proc_macro::TokenStream,
	) -> syn::Result<TokenStream> {
		let func = syn::parse::<ItemFn>(input)?;

		let is_async = func.sig.asyncness.is_some();

		// non async tests are just #[test]
		if !is_async {
			let out = quote! {
				#[test]
				#func
			}
			.into();
			return Ok(out);
		}

		let func_inner = wrap_func_inner(&func)?;

		let attrs = &func.attrs;
		let vis = &func.vis;
		let sig = &func.sig;
		let name = &sig.ident;
		let out = quote! {
				#[test]
				#(#attrs)*
				#vis fn #name() {
						sweet::sweet_test::SweetTestCollector::register(Box::pin(#func_inner));
				}
		};

		Ok(out)
	}
}


fn wrap_func_inner(func: &ItemFn) -> syn::Result<TokenStream> {
	let body = &func.block;

	match &func.sig.output {
		ReturnType::Default => Ok(quote! {
			async #body
		}),
		ReturnType::Type(_, ty) => {
			if !returns_result(ty) {
				return Err(syn::Error::new_spanned(
					ty,
					"async test functions must return Unit or Result",
				));
			}
			// println!("RFSDDSSD {}", ty);
			// let ty = ty.to_token_stream();
			Ok(quote! {
				async {
					let result:#ty = async #body.await;
					if let Err(e) = result {
							panic!("{:?}", e);
					}
				}
			})
		}
	}
}

fn returns_result(ty: &Box<Type>) -> bool {
	match &**ty {
		syn::Type::Path(type_path) => {
			let segments = &type_path.path.segments;
			if let Some(last_segment) = segments.last() {
				last_segment.ident == "Result"
			} else {
				false
			}
		}
		_ => false,
	}
}

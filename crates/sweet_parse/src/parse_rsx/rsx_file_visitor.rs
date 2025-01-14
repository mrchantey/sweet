use crate::prelude::*;
use quote::ToTokens;
use syn::visit;
use syn::visit_mut;
use syn::visit_mut::VisitMut;
use syn::Expr;
use syn::ExprMacro;
use syn::ForeignItemMacro;
use syn::Stmt;



/// The rsx visitor is used by file (preprocessor) parsers.
pub struct RsxFileVisitor<'a, P> {
	plugin: &'a mut P,
	/// The rsx macros found in the function
	macros: Vec<WalkNodesOutput>,
	/// Errors that occurred while parsing the rsx macro
	errors: Vec<syn::Error>,
}

/// Output from a fully parsed file with multiple rsx macros.
pub struct RsxFileVisitorOut {
	pub macros: Vec<WalkNodesOutput>,
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
	pub fn extend_result(
		&mut self,
		result: Result<RsxFileVisitorOut, syn::Error>,
	) {
		match result {
			Ok(out) => {
				self.macros.extend(out.macros);
				self.errors.extend(out.errors);
			}
			Err(e) => self.errors.push(e),
		}
	}
}

impl<'a, P: RsxPlugin> VisitMut for RsxFileVisitor<'a, P> {
	fn visit_macro_mut(&mut self, item: &mut syn::Macro) {
		// println!("item {:?}", item);
		if P::path_matches(&item.path) {
			match self.plugin.visit_rsx(item) {
				Ok(parts) => self.macros.push(parts),
				Err(e) => self.errors.push(e),
			}
			// place path::to::rsx! with noop!
			item.path = syn::parse_quote!(sweet::noop)
		}
		// visit nested
		visit_mut::visit_macro_mut(self, item);
	}


	fn visit_stmt_mut(&mut self, stmt: &mut Stmt) {
		// match &stmt {
		// 	Stmt::Macro(stmt_mac) => {}
		// 	Stmt::Expr(_, _) => {
		// 		println!("expr {}", stmt.to_token_stream().to_string());
		// 	}
		// 	stmt => {
		// 		println!("item {}", stmt.to_token_stream().to_string());
		// 	}
		// }
		// visit nested
		visit_mut::visit_stmt_mut(self, stmt);
	}

	// fn visit_macro_delimiter_mut(&mut self, item: &mut syn::MacroDelimiter) {
	// 	println!("item {:?}", item);
	// }
	// fn visit_item_macro_mut(&mut self, item: &mut syn::ItemMacro) {
	// 	println!("item {:?}", item);
	// }
	// fn visit_type_macro_mut(&mut self, item: &mut syn::TypeMacro) {
	// 	println!("item {:?}", item);
	// }
	// fn visit_trait_item_macro_mut(&mut self, item: &mut syn::TraitItemMacro) {
	// 	println!("item {:?}", item);
	// }

	// fn visit_expr_mut(&mut self, expr: &mut Expr) {
	// 	println!("expr {:?}", expr);
	// }
	// fn visit_expr_macro_mut(&mut self, expr: &mut ExprMacro) {
	// 	println!("expr {:?}", expr);
	// }
	// fn visit_foreign_item_macro_mut(&mut self, expr: &mut ForeignItemMacro) {
	// 	println!("expr {:?}", expr);
	// }
}

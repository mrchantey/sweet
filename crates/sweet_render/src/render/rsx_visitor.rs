use sweet_core::error::ParseResult;
use sweet_core::rsx::Element;
use sweet_core::rsx::Node;
use sweet_core::rsx::RustParts;







#[allow(unused_variables)]
pub trait RsxVisitor {
	/// Useful for keeping track of the position of the RustParts vec.
	/// Note that mutating events will have no effect as the renderer will discard them
	fn visit_rust(&mut self, rust: &mut RustParts) -> ParseResult<()> { Ok(()) }

	fn visit_node(&mut self, node: &mut Node<RustParts>) -> ParseResult<()> {
		Ok(())
	}
	/// Non-component elements
	fn visit_element(
		&mut self,
		element: &mut Element<RustParts>,
	) -> ParseResult<()> {
		Ok(())
	}
	fn visit_component(
		&mut self,
		element: &mut Element<RustParts>,
	) -> ParseResult<()> {
		Ok(())
	}

	/// visit an attribute that is an event, return the html attribute
	/// that should be used for the event
	fn visit_event_attribute(&mut self, key: &str) -> ParseResult<String>;

	fn visit_text_block(&mut self, key: &mut String) -> ParseResult<()> {
		Ok(())
	}

	// fn visit_final(&mut self, out: &mut RsxRendererOut) -> ParseResult<()> {
	// 	Ok(())
	// }
}

use sweet_core::error::ParseResult;
use sweet_core::rsx::Element;
use sweet_core::rsx::Node;
use sweet_core::rsx::RsxRust;







#[allow(unused_variables)]
pub trait RsxVisitor {
	/// Useful for keeping track of the position of the RsxRust vec.
	/// Note that mutating events will have no effect as the renderer will discard them
	fn visit_rust(&mut self, rust: &mut RsxRust) -> ParseResult<()> { Ok(()) }

	fn visit_node(&mut self, node: &mut Node) -> ParseResult<()> { Ok(()) }
	/// Non-component elements
	fn visit_element(&mut self, element: &mut Element) -> ParseResult<()> {
		Ok(())
	}
	fn visit_component(&mut self, element: &mut Element) -> ParseResult<()> {
		Ok(())
	}
	fn visit_event_attribute(
		&mut self,
		key: &mut String,
		value: &mut String,
	) -> ParseResult<()> {
		Ok(())
	}

	fn visit_block_value_attribute(
		&mut self,
		key: &mut String,
	) -> ParseResult<()> {
		Ok(())
	}
}



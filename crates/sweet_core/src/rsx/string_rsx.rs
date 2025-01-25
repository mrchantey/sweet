use super::RsxRust;

/// A simple rsx implementation that
/// calls to_string on all rsx parts
#[derive(Debug)]
pub struct StringRsx;

impl RsxRust for StringRsx {
	type Block = String;
	type AttributeBlock = String;
	type AttributeBlockValue = String;
	fn attribute_block_to_string(block: &Self::AttributeBlock) -> String {
		block.clone()
	}
	fn attribute_block_value_to_string(
		block: &Self::AttributeBlockValue,
	) -> String {
		block.clone()
	}
	fn block_to_string(block: &Self::Block) -> String { block.clone() }
}

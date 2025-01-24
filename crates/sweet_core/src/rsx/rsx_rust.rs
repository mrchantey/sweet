use super::RustParts;

pub trait RsxRust {
	type Block;
	type AttributeBlock;
	type AttributeBlockValue;

	fn block_to_string(block: &Self::Block) -> String;
	fn attribute_block_to_string(block: &Self::AttributeBlock) -> String;
	fn attribute_block_value_to_string(
		block: &Self::AttributeBlockValue,
	) -> String;
}

impl RsxRust for RustParts {
	type Block = String;
	type AttributeBlock = String;
	type AttributeBlockValue = String;

	fn block_to_string(block: &Self::Block) -> String { block.clone() }
	fn attribute_block_to_string(block: &Self::AttributeBlock) -> String {
		block.clone()
	}
	fn attribute_block_value_to_string(
		block: &Self::AttributeBlockValue,
	) -> String {
		block.clone()
	}
}


impl RsxRust for () {
	type Block = ();
	type AttributeBlock = ();
	type AttributeBlockValue = ();
	fn attribute_block_to_string(_block: &Self::AttributeBlock) -> String {
		String::from("()")
	}
	fn attribute_block_value_to_string(
		_block: &Self::AttributeBlockValue,
	) -> String {
		String::from("()")
	}
	fn block_to_string(_block: &Self::Block) -> String { String::from("()") }
}
impl RsxRust for String {
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

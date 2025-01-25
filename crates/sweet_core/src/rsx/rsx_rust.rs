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

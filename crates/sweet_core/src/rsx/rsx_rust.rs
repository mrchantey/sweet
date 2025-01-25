pub trait RsxRust {
	type NodeBlock;
	type AttributeBlock;
	type AttributeBlockValue;

	fn block_to_string(block: &Self::NodeBlock) -> String;
	fn attribute_block_to_string(block: &Self::AttributeBlock) -> String;
	fn attribute_block_value_to_string(
		block: &Self::AttributeBlockValue,
	) -> String;
}

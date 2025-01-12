use super::collect_nodes;
use super::empty_elements;
use super::parse_nodes;
use super::WalkNodes;

pub struct RsxMacro;


impl RsxMacro {
	pub fn parse(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
		let (nodes, errors) = parse_nodes(tokens);

		let output = WalkNodes::walk_nodes(empty_elements(), nodes);

		collect_nodes(output, errors)
	}
}

#[cfg(test)]
mod test {
	use sweet::prelude::*;

	#[test]
	fn events() {
		// macro with an event and block
		// raw text nodes are trimmed
		let mut tokens = quote::quote! {
			<div onclick></div>
		};

		let out = RsxParser::default().parse_rsx(&mut tokens).unwrap();
		let tokens_str = tokens.to_string();
		// let tokens_str = prettyplease::unparse(
		// 	&syn::parse_file(&tokens.to_string()).unwrap(),
		// );
		// println!("{}", tokens_str);

		expect(&tokens_str).to_contain("(onclick)");
		expect(&tokens_str).not().to_start_with("rsx!");
		expect(&out.html).to_be(r#"<div § onclick="§"></div>"#);
	}
	#[test]
	fn text_blocks() {
		// raw text nodes `is` are trimmed
		let mut tokens = quote::quote! {
			<div>"the "{value}"th "<bold>value</bold> is {value}</div>
		};
		let out = RsxParser::default().parse_rsx(&mut tokens).unwrap();
		expect(out.html).to_be(r#"<div §>the §th <bold>value</bold>is§</div>"#);
	}
	#[test]
	#[ignore]
	fn child_component() {
		// raw text nodes are trimmed
		let mut tokens = quote::quote! {
			rsx!{<body><Header>"sweet "<b/>as</b></Header></body>}
		};
		let out = RsxParser::default().parse_rsx(&mut tokens).unwrap();
		expect(out.html).to_be(r#"<body>§§sweet <b>as</>§§</body>"#);
	}
}

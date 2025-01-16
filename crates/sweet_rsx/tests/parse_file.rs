#![cfg_attr(test, feature(test, custom_test_frameworks))]
#![cfg_attr(test, test_runner(sweet_test::test_runner))]

#[cfg(test)]
#[cfg(not(target_arch = "wasm32"))]
mod test {
	use forky::prelude::*;
	use sweet_rsx::prelude::*;
	use sweet_test::prelude::*;

	#[test]
	fn works() {
		let file = include_str!("../../../examples/counter.rs");

		let (file, out) = RsxParser::default().parse_file(file).unwrap();
		let file_str = prettyplease::unparse(&file);

		let path =
			FsExt::workspace_root().join("target/rendered/counter_parsed.rs");
		ReadFile::write(path, &file_str).unwrap();

		expect(out.errors.len()).to_be(0);
		expect(out.macros.len()).to_be(3);
		expect(&file_str).to_start_with(RsxParser::SHEBANG);
		expect(&file_str).not().to_contain("rsx!");
	}
}

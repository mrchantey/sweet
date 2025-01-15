#![cfg_attr(test, feature(test, custom_test_frameworks))]
#![cfg_attr(test, test_runner(sweet::test_runner))]

#[cfg(test)]
#[cfg(not(target_arch = "wasm32"))]
mod test {
	use forky::prelude::FsExt;
	use forky::prelude::ReadFile;
	use sweet::prelude::*;

	#[test]
	fn works() {
		let file = include_str!("../../../examples/counter.rs");

		let (file, out) = RsxParser::default().parse_file(file).unwrap();
		let file_str = prettyplease::unparse(&file);

		let path = FsExt::workspace_root().join("examples/counter_parsed.rs");
		ReadFile::write(path, &file_str).unwrap();

		expect(out.errors.len()).to_be(0);
		expect(out.macros.len()).to_be(3);
		expect(&file_str).to_start_with(RsxParser::SHEBANG);
		expect(&file_str).not().to_contain("rsx!");
	}
}

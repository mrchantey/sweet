#![cfg_attr(test, feature(test, custom_test_frameworks))]
#![cfg_attr(test, test_runner(sweet::test_runner))]

#[cfg(test)]
mod test {
	use sweet::prelude::*;

	#[test]
	fn works() {
		let parts: RsxParts = rsx! {<div> the value is {3} </div>};
		let parts_str = format!("{:?}", parts);

		expect(parts_str).to_be("RsxParts { rust count: 1, html: Inline(\"<div rsx-id=\\\"0\\\"> the value is  §</div>\"), css: Inline(\"\") }");
	}
}

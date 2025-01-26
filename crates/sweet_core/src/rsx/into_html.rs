


#[cfg(test)]
mod test {
	use crate::prelude::*;

	#[test]
	fn doctype() {
		let out = DefaultRsxRenderer::render(rsx! { <!DOCTYPE html> }).unwrap();
		expect(out).to_be("<!DOCTYPE html>");
	}

	#[test]
	fn comment() {
		let out =
			DefaultRsxRenderer::render(rsx! { <!-- "hello" --> }).unwrap();
		expect(out).to_be("<!-- hello -->");
	}

	#[test]
	fn text() {
		let out = DefaultRsxRenderer::render(rsx! { "hello" }).unwrap();
		expect(out).to_be("hello");
	}

	#[test]
	fn element() {
		let key = "hidden";
		let key_value = "class=\"pretty\"";
		let food = "pizza";
		let out = DefaultRsxRenderer::render(rsx! { <div
			name="pete"
			age=9
			{key}
			{key_value}
			favorite_food={food}
			>
			</div>
		})
		.unwrap();
		expect(out).to_be("<div name=\"pete\" age=\"9\" hidden class=\"pretty\" favorite_food=\"pizza\" data-sweet-id=\"0\"></div>");
	}
	#[test]
	fn element_self_closing() {
		let out = DefaultRsxRenderer::render(rsx! { <br/> }).unwrap();
		expect(out).to_be("<br/>");
	}
	#[test]
	fn element_children() {
		let out =
			DefaultRsxRenderer::render(rsx! { <div>hello</div> }).unwrap();
		expect(out).to_be("<div>hello</div>");
	}

	#[test]
	fn text_block() {
		let value = "hello";
		let out = DefaultRsxRenderer::render(rsx! { {value} }).unwrap();
		expect(out).to_be("hello");
	}

	#[test]
	fn component() {
		struct Child {
			value: u32,
		}
		impl Component for Child {
			fn render(self) -> impl Rsx {
				rsx! {
					<div>{self.value}</div>
				}
			}
		}
		let out =
			DefaultRsxRenderer::render(rsx! { <Child value=7/> }).unwrap();
		expect(out).to_be(
			"<div data-sweet-id=\"1\" data-sweet-blocks=\"0-0-1\">7</div>",
		);
	}


	#[test]
	fn nested() {
		let onclick = |_| {};
		let world = "mars";
		let rsx = rsx! {
			<div onclick>
				<p>hello {world}</p>
			</div>
		};
		// println!("rsx: '{:#?}'", rsx);
		let out = DefaultRsxRenderer::render(rsx).unwrap();
		expect(out).to_be("<div onclick=\"_sweet_event\" data-sweet-id=\"0\"><p data-sweet-id=\"1\" data-sweet-blocks=\"0-6-4\">hello mars</p></div>");
	}
}

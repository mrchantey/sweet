use crate::matchers::*;
use crate::prelude::BacktraceResult;
use crate::prelude::BuildableResult;
use anyhow::Result;
use forky::web::*;
use web_sys::*;

pub trait MatcherHtml<T>: MatcherTrait<T>
where
	T: AsRef<HtmlElement>,
{
	fn get(&self, selector: &str) -> Result<Matcher<HtmlElement>> {
		let matcher = self.get_matcher();
		let parent = matcher.value.as_ref();
		// let expected = format!(
		// 	"element {} to contain selector '{selector}'",
		// 	parent.tag_name()
		// );
		let received = parent.x_query_selector::<HtmlElement>(selector);
		matcher
			.assert_option_with_received(received)
			.map(|c| Matcher::new(c))
			.build_res_mapped()
	}

	fn to_contain_text(&self, other: &str) -> Result<()> {
		let receive =
			self.get_value().as_ref().text_content().unwrap_or_default();
		self.contains(other, &receive, "text").build_res_mapped()
	}
	fn to_contain_visible_text(&self, other: &str) -> Result<()> {
		let receive = self.get_value().as_ref().inner_text();
		self.contains(other, &receive, "visible text")
			.build_res_mapped()
	}
	fn to_contain_html(&self, other: &str) -> Result<()> {
		let receive = self.get_value().as_ref().inner_html();
		self.contains(other, &receive, "html").build_res_mapped()
	}
	fn contains(
		&self,
		other: &str,
		receive: &str,
		expect_suffix: &str,
	) -> BacktraceResult {
		let result = receive.contains(other);
		// forky::core::log!("result: {result}");
		let mut received = receive.chars().take(100).collect::<String>();
		if received.len() == 100 {
			received.push_str("...TRUNCATED...");
		}
		let expected = format!("to contain {} '{}'", expect_suffix, other);

		self.get_matcher()
			.assert_correct_with_received(result, &expected, &received)
	}
}

impl<T> MatcherHtml<T> for Matcher<T> where T: AsRef<HtmlElement> {}

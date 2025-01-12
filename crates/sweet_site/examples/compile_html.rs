use sweet_rsx::rsx::IntoHtml;
use sweet_site::pages::Index;

pub fn main() {
	let html = Index::into_html().unwrap();

	println!("{}", html);
}

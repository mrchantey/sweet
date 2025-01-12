pub fn main() {
	let html = sweet_site::out::pages::index::render_html().unwrap();

	println!("{}", html);
}

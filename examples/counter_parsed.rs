use sweet::prelude::*;
fn main() {
    Server::default().run(());
    Server::default().run_once(Counter { value: 7 });
    Server::default().run_once(Footer);
    Server::default()
        .run_once(
            sweet::noop! {
                { sweet::prelude::RsxParts { rust : vec![], html :
                PathOrInline::Inline("<Counter></Counter>".to_string()), css :
                PathOrInline::Inline("".to_string()), } }
            },
        );
}
struct Counter {
    value: i32,
}
impl Rsx for Counter {
    fn into_parts(self) -> RsxParts {
        sweet::noop! {
            { sweet::prelude::RsxParts { rust : vec![sweet::prelude::RsxRust::InnerText({
            self.value } .to_string()), RsxRust::Event(Box::new(| _ | {})),], html :
            PathOrInline::Inline("<div rsx-id=\"0\"> the value is  §</div><button rsx-id=\"1\" onclick=\"§\">Increment</button><Footer></Footer>"
            .to_string()), css : PathOrInline::Inline("".to_string()), } }
        }
    }
}
struct Footer;
impl Rsx for Footer {
    fn into_parts(self) -> RsxParts {
        sweet::noop! {
            { sweet::prelude::RsxParts { rust : vec![], html :
            PathOrInline::Inline("<footer><div>sweet as!</div></footer>".to_string()),
            css : PathOrInline::Inline("".to_string()), } }
        }
    }
}

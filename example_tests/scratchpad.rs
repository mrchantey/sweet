#![cfg_attr(test, feature(test, custom_test_frameworks))]
#![cfg_attr(test, test_runner(sweet::test_runner))]

extern crate test;
use test::Bencher;
#[bench]
fn main(b: &mut Bencher) {
	b.iter(|| {
		println!("hello");
	});
}

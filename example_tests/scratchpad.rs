#![cfg_attr(test, feature(test, custom_test_frameworks))]
#![cfg_attr(test, test_runner(sweet::test_runner))]

// extern crate test;
// use test::Bencher;
// #[bench]
// fn main(b: &mut Bencher) {
// 	b.iter(|| {
// 		println!("hello");
// 	});
// }
// #[tokio::test]
// async fn my_test() {

	
// }

fn process<T, U>(value: U) -> T 
where
    U: Into<T> + AsRef<T>
{
    if std::mem::size_of::<U>() == std::mem::size_of::<&T>() {
        value.as_ref().clone()
    } else {
        value.into()
    }
}

// Example usage:
fn main() {
    let s = String::from("hello");
    let result1: String = process(s.clone()); // Takes owned String
    let result2: String = process(&s);        // Takes &String
    println!("{} {}", result1, result2);
}
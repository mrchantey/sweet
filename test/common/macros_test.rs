use sweet::sweet_test;
use sweet::prelude::*;

#[sweet_test]
fn test_macro_compiles() -> Result<()> {
    Ok(())
}

#[sweet_test(skip)]
fn skips() -> Result<()> {
    expect(true).to_be_false()
}
#[sweet_test]
#[ignore]
fn ignores_foo() {
    panic!()
}

// #[sweet_test(skip, only)]
// fn skips_only() -> Result<()> {
//     expect(true).to_be_false()
// }

#[sweet_test(e2e)]
fn e2e() -> Result<()> {
    Ok(())
}

#[cfg(test)]
mod test {
    use sweet::sweet_test;
    // use sweet::prelude::*;
    #[sweet_test]
    fn checks_inside_mod() {
        // panic!()
    }
}

// #[cfg(not(target_arch = "wasm32"))]
// #[sweet_test(e2e, non_send)]
// async fn can_be_async() -> Result<()> {
//     tokio::time::sleep(std::time::Duration::from_millis(1)).await;
//     Ok(())
// }

// #[cfg(not(target_arch = "wasm32"))]
// #[sweet_test(non_send)]
// async fn can_be_async_non_send() -> Result<()> {
//     tokio::time::sleep(std::time::Duration::from_millis(1)).await;
//     Ok(())
// }

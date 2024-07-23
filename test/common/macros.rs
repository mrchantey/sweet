use sweet::*;

#[sweet::test]
fn sweet_test_macro_compiles() -> Result<()> {
    Ok(())
}

#[sweet::test(skip)]
fn skips() -> Result<()> {
    expect(true).to_be_false()
}
#[sweet::test]
#[ignore]
fn ignores_foo() {
    panic!()
}

// #[sweet::test(skip, only)]
// fn skips_only() -> Result<()> {
//     expect(true).to_be_false()
// }

#[sweet::test(e2e)]
fn e2e() -> Result<()> {
    Ok(())
}

#[cfg(test)]
mod test {
    // use sweet::*;
    #[sweet::test]
    fn checks_inside_mod() {
        // panic!()
    }
}

// #[cfg(not(target_arch = "wasm32"))]
// #[sweet::test(e2e, non_send)]
// async fn can_be_async() -> Result<()> {
//     tokio::time::sleep(std::time::Duration::from_millis(1)).await;
//     Ok(())
// }

// #[cfg(not(target_arch = "wasm32"))]
// #[sweet::test(non_send)]
// async fn can_be_async_non_send() -> Result<()> {
//     tokio::time::sleep(std::time::Duration::from_millis(1)).await;
//     Ok(())
// }

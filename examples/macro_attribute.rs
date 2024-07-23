pub use sweet::*;

#[sweet::test]
fn assert_test() {
    assert!(true)
}
#[sweet::test]
fn result_test() -> anyhow::Result<()> {
    expect(true).to_be_true()?;
    Ok(())
}
#[sweet::test]
async fn async_assert_test() {
    assert!(true)
}
#[sweet::test]
async fn async_result_test() -> anyhow::Result<()> {
    expect(true).to_be_true()?;
    Ok(())
}

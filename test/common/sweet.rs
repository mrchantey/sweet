use sweet::*;

#[sweet_test]
async fn works() -> Result<()> {
    async fn foobar() {}
    foobar().await;

    // println!("foobar");
    expect(true).to_be_true()?;

    Ok(())
}

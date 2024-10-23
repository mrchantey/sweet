use anyhow::*;
use sweet::native::*;
// use std::pin::Pin;
use sweet::*;

#[sweet_test]
async fn panic_block()-> Result<()> {
    std::panic::set_hook(Box::new(|_| {}));
    let func = Box::pin(async { panic!("hello");});
    let result = unwrap_panic_async(func).await;
    expect(result).to_be_err_str("hello")?;
    let _ = std::panic::take_hook();
    
    Ok(())
}
#[sweet_test]
async fn panic_func()-> Result<()> {
    std::panic::set_hook(Box::new(|_| {}));
    let func = || Box::pin(async { panic!("hello");});
    let result = unwrap_panic_async((func)()).await;
    expect(result).to_be_err_str("hello")?;
    let _ = std::panic::take_hook();
    
    Ok(())
}
#[sweet_test]
async fn error()-> Result<()> {
    let func = || Box::pin(async { Err(anyhow!("hello"))});
    let result = unwrap_panic_async((func)()).await;
    expect(result).to_be_err_str("hello")?;
    
    Ok(())
}
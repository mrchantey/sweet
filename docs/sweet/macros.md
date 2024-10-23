# Macros

# `#[sweet_test]`

Tests can be declared via an attribute.

```rs
#[sweet_test]
fn foobar(){}

//accepts several flags, async functions or an `anyhow::Result` return type
#[sweet_test(skip,only,e2e,non_send)]
async fn foobar()->Result<()>{
	expect(true).to_be_true()
}
```
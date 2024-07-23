# CI / CD

Sweet has full CI/CD support for all test types. In fact, the [tests for this repo][1] are all run using Github Actions.

[1]:https://github.com/mrchantey/forky/blob/main/.github/workflows/test.yml

An example workflow may look something like [this](https://github.com/mrchantey/sweet-demo/blob/main/.github/workflows/test.yml):

```yaml
name: 🔎 Test Crates
on:
  push:
    branches: main
  pull_request:
    branches: main
env:
  CARGO_TERM_COLOR: always
jobs:
  build_and_test:
    name: Build and Test
    runs-on: ubuntu-latest
    steps:
    - name: 📂 Checkout
      uses: actions/checkout@v3
    - name: 📂 Cache
      uses: actions/cache@v3
      with:
        path: |
          ~/.cargo/bin/
          ~/.cargo/registry/index/
          ~/.cargo/registry/cache/
          ~/.cargo/git/db/
          target/
        key: ${{ runner.os }}-cargo-${{ hashFiles('**/Cargo.lock') }}
    - name: 🔨 Install Chromedriver
      uses: nanasess/setup-chromedriver@v2
    - name: 🔨 Install Rust
      uses: actions-rs/toolchain@v1
      with:
        toolchain: nightly
        override: true
        default: true
    - name: 🔨 Install Wasm Target
      run: rustup target add wasm32-unknown-unknown
    - name: 🔨 Install Wasm Bindgen
      uses: baptiste0928/cargo-install@v2
      with:
        crate: wasm-bindgen-cli
        version: '0.2.87' # TODO ensure this matches your wasm-bindgen version
    - name: 🔨 Install Sweet Cli
      uses: baptiste0928/cargo-install@v2
      with:
        crate: sweet-cli
    - name: 🔨 Build
      run: cargo build
    - name: 🔎 Test Native
      run: cargo run --example sweet
    - name: 🔎 Test Wasm
      run: sweet --example sweet
```

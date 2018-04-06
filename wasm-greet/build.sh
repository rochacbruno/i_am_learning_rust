#!/bin/sh

set -ex

rustup target add wasm32-unknown-unknown --toolchain nightly

# Build the `hello_world.wasm` file using Cargo/rustc
cargo +nightly build --target wasm32-unknown-unknown

# Run the `wasm-bindgen` CLI tool to postprocess the wasm file emitted by the
# Rust compiler to emit the JS support glue that's necessary

cargo install wasm-bindgen-cli
wasm-bindgen target/wasm32-unknown-unknown/debug/wasm_greet.wasm --out-dir .

# Finally, package everything up using Webpack and start a server so we can
# browse the result
npm install
npm run serve

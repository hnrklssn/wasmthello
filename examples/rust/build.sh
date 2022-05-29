#!/bin/sh
cargo wasi build --release
# cargo fails to find wasm-opt on my machine for some reason
wasm-opt target/wasm32-wasi/release/wasmthello_rust_bot.rustc.wasm -Oz --strip-debug -o hello.wasm
wasm2wat hello.wasm -o hello.wat

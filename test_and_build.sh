#!/usr/bin/env bash

cargo build --all-targets --all --examples
cargo test
cargo run --example example-with-move
cargo run --example interval-and-timeout-like-in-javascript
cargo run --example macro-possible-arguments
cargo run --example minimal

cd "external-example" || exit;
cargo build
cd ..



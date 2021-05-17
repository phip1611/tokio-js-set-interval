#!/usr/bin/env bash

cargo build --all-targets --all --examples
cargo test

cd "external-example" || exit;
cargo build
cd ..



#!/bin/bash
cargo build --release --offline --quiet --manifest-path=./playground/Cargo.toml
cp ./playground/target/release/main ./playground/a.out

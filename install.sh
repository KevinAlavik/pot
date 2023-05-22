‹#!/bin/bash
# Build the Rust program
cargo build --release
# Move the output target to /usr/local/bin/pot
sudo mv target/release/pot /usr/local/bin/
# Remove the target directory
cargo clean

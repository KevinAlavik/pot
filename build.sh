#!/bin/bash
cargo build --release
sudo mv target/release/pot /usr/local/bin/
cargo clean
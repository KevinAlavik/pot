#!/bin/bash
cargo build --release
sudo mv target/release/pot /usr/local/bin/
sudo cargo clean
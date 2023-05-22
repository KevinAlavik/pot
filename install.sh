‹#!/bin/bash
# Clone the repo
git clone https://github.com/kevinalavik/pot
# Go inside of the repo
cd pot
# Build the Rust program
cargo build --release
# Move the output target to /usr/local/bin/pot
sudo mv target/release/pot /usr/local/bin/
# Remove the target directory
cargo clean
# Move out and clean
cd ..
sudo rm -rf pot/

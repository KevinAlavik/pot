#!/bin/bash
# Clone the repo
git clone https://github.com/kevinalavik/pot
# Go inside of the repo
cd pot
# Build the Rust program
cargo build --release
# Move the output target to /usr/local/bin/pot
sudo mv target/release/pot /usr/local/bin/
# Remove the target directory
sudo cargo clean
# Move out and clean
cd ..
sudo rm -rf pot/
<<<<<<< HEAD
sudo mkdir /etc/pot/
sudo touch /etc/pot/sources.json
sudo echo '[ "https://puffer.is-a.dev/pot/pot.json" ]' > /etc/pot/sources.json
=======
echo '[ "https://puffer.is-a.dev/pot/pot.jdon" ]' > /etc/pot/sources.json
>>>>>>> 8967f013e97f282a08e752bb45681c9880f7a1be

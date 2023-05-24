#!/bin/bash
cargo build --release
sudo mv target/release/pot /usr/local/bin/
cargo clean
sudo mkdir /etc/pot/
sudo touch /etc/pot/sources.json
sudo echo '[ "https://puffer.is-a.dev/pot/pot.json" ]' > /etc/pot/sources.json
#!/bin/bash
cargo build

cd ../../
cat tron.toml
echo 'name = "Tron_App"
entry = "main"
version = "0.0.1"
authors = "you"
license = "MIT"
decor = "default"
pointer = "default"
env = "prod"
experimental = "false"
credits = "false"
warnings = "true"' > tron.toml
cat main.tron
echo 'print "Hello, World!";' > main.tron
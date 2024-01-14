#!/bin/bash

echo "Installing Tron for Unix..."
curl -o tron https://tronlang.org/tron-lang && \
sudo mv tron /usr/local/bin/ && \
sudo chmod +x /usr/local/bin/tron
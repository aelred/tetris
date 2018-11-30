#!/bin/bash

# fail immediately if a command fails
set -e
set -o pipefail

cd tetris-server
cargo install --force
sudo systemctl restart tetris

#!/bin/bash

# fail immediately if a command fails
set -e
set -o pipefail

cargo install --path tetris-server
sudo systemctl restart tetris

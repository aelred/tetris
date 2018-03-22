#!/bin/bash
cd tetris-server
cargo install --force
sudo systemctl restart tetris

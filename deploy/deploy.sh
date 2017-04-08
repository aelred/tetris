#!/bin/bash

# fail immediately if a command fails
set -e
set -o pipefail

cd ~/sites/tetris

git fetch --all
git reset --hard origin/master

deploy/js.sh
deploy/server.sh
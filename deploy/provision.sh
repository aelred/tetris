#!/bin/bash

sudo apt-get install cmake

# install emscripten
mkdir ~/emsdk
pushd ~/emsdk
if [ ! -e epsdk-portable.tar.gz ]; then
    wget https://s3.amazonaws.com/mozilla-games/emscripten/releases/emsdk-portable.tar.gz
fi

if [ ! -d emsdk-portable ]; then
    tar -xvf emsdk-portable.tar.gz
fi
cd emsdk-portable
./emsdk update
./emsdk install sdk-incoming-64bit
popd

rustup target add asmjs-unknown-emscripten

SITENAME=tetris

sudo cp ./nginx.conf /etc/nginx/sites-available/$SITENAME
sudo ln -s /etc/nginx/sites-available/$SITENAME /etc/nginx/sites-enabled/$SITENAME
sudo service nginx reload

if [ ! -d ~/sites/tetris ]; then
    git clone git@github.com:aelred/tetris.git ~/sites/tetris/
fi

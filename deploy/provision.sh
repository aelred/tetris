#!/bin/bash

SITENAME=tetris

sudo cp ./nginx.conf /etc/nginx/sites-available/$SITENAME
sudo ln -s /etc/nginx/sites-available/$SITENAME /etc/nginx/sites-enabled/$SITENAME
sudo service nginx reload

git clone git@github.com:aelred/tetris.git ~/sites/tetris/

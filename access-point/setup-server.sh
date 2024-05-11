#!/bin/bash
useradd liminalbox
mkdir /liminalbox
mkdir /liminalbox/files
touch /liminalbox/chat.json
chmod +w /liminalbox/chat.json
chown liminalbox /liminalbox -R
cp -r ../target/site /liminalbox/
cp ../target/release/liminalbox /usr/bin/
cp ./liminalbox.service /etc/systemd/system/
systemctl enable liminalbox
systemctl start liminalbox

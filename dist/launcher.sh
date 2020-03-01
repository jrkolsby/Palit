#!/bin/sh
rm -f /tmp/pt-client
rm -f /tmp/pt-sound
mkfifo /tmp/pt-client
mkfifo /tmp/pt-sound
touch ./logs/pt-sound
touch ./logs/pt-client
sudo ./bin/pt-input 1> /tmp/pt-client 2> /tmp/pt-sound &
./bin/pt-sound >> ./logs/pt-sound 2>&1 &
./bin/pt-client 2>>./logs/pt-client

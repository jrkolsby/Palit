#!/bin/sh
sudo ./bin/pt-input 1> /tmp/pt-client 2> /tmp/pt-sound &
./bin/pt-sound &
./bin/pt-client 2> /tmp/pt-debug

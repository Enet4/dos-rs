#!/usr/bin/env bash
set -eu

# download from https://github.com/andrewwutw/build-djgpp
curl -L https://github.com/andrewwutw/build-djgpp/releases/download/v3.4/djgpp-linux64-gcc1220.tar.bz2 -o djgpp-linux64-gcc1220.tar.bz2

# extract bz2 file to /opt/djgpp
tar -xvf djgpp-linux64-gcc1220.tar.bz2 -C /opt

# clean up
rm djgpp-linux64-gcc1220.tar.bz2

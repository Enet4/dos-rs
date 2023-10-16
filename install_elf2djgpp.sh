#!/usr/bin/env bash
set -eu

git clone --depth 1 https://github.com/cknave/elf2djgpp.git --branch main
cd elf2djgpp
REV=$(git rev-parse HEAD)
if [ -f "~/.cargo/bin/elf2dgjpp" ]; then
    OLD_REV=$(cat ~/.cargo/bin/elf2dgjpp-rev.txt)
    if [ "$REV" = "$OLD_REV" ]; then
        echo "elf2djgpp is up to date"
        cd ..
        # clean up
        rm -rf elf2djgpp
        exit 0
    fi
fi
cargo install --path .
git rev-parse HEAD > ~/.cargo/bin/elf2dgjpp-rev.txt
cd ..
# clean up
rm -rf elf2djgpp

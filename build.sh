#!/usr/bin/env sh
set -eu
#RUSTFLAGS='-C code-model=large'

cargo clean
RUST_TARGET_PATH=`pwd` xargo build
RUST_TARGET_PATH=`pwd` xargo build --release
i686-pc-msdosdjgpp-objdump -D target/i386-pc-msdos-djgpp/debug/dos-rs.exe > all-dos-rs.asm
i686-pc-msdosdjgpp-objdump -t target/i386-pc-msdos-djgpp/debug/dos-rs.exe > dump.txt
rg '.*scl.* _([_\.\w]+)$' dump.txt > _symbols.txt
sort _symbols.txt > symbols.txt
rm -f _symbols.txt

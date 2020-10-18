#!/usr/bin/env sh
set -eu

cargo clean
RUST_TARGET_PATH=`pwd` xargo rustc -- --emit obj
i686-pc-msdosdjgpp-gcc \
    --verbose \
    -m32 \
    -march=i386 \
    -fno-pie \
    -fno-use-linker-plugin \
    -Wl,--as-needed \
    -Wl,--gc-sections \
    -Wl,--start-group \
    -Ltarget/i386-pc-msdos-djgpp/debug/deps \
    -Wl,--end-group \
    target/i386-pc-msdos-djgpp/debug/deps/*.o \
    -o target/i386-pc-msdos-djgpp/debug/dos-rs.exe
#RUST_TARGET_PATH=`pwd` xargo build
cp target/i386-pc-msdos-djgpp/debug/dos-rs.exe .dos-rs.exe
./coff-to-djgpp.pl < .dos-rs.exe > dos-rs-n.exe
i686-pc-msdosdjgpp-objdump -D .dos-rs.exe > all-dos-rs.asm
i686-pc-msdosdjgpp-objdump -t .dos-rs.exe > dump.txt
i686-pc-msdosdjgpp-objdump -D dos-rs-n.exe > all-dos-rs.new.asm
i686-pc-msdosdjgpp-objdump -t dos-rs-n.exe > dump.new.txt
rg '.*scl.* _([_\.\w]+)$' dump.txt > _symbols.txt
sort _symbols.txt > symbols.txt
rm -f _symbols.txt

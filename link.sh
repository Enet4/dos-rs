#!/usr/bin/env bash
i686-pc-msdosdjgpp-gcc \
    --verbose \
    -m32 -march=i386 \
    -fno-pie -fno-use-linker-plugin \
    -Wl,--as-needed \
    -Wl,--gc-sections \
    $* \
    -o RS-MAIN.exe

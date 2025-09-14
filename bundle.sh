#!/usr/bin/env sh
set -eu

# only call build.sh if the file was not built yet
if [ ! -f ./build/release/ferris.exe ]; then
    ./build.sh release
fi
if [ ! -f ./build/release/opl.exe ]; then
    ./build-opl.sh release
fi
# add ferris.exe, assets, dosbox.conf and CWSDPMI.EXE into new zip file
cd bundle
rm -f dos_rs.zip
cp ../build/release/ferris.exe ../build/release/opl.exe ./
zip -q dos_rs.zip \
    CWSDPMI.EXE \
    ferris.exe \
    opl.exe \
    abnormal.vgm \
    wares.vgm \
    .jsdos/dosbox.conf
rm ferris.exe opl.exe

# move it as dos_rs.jsdos
cp dos_rs.zip ../dos_rs.jsdos
rm dos_rs.zip
echo "Created bundle dos_rs.jsdos"

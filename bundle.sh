#!/usr/bin/env sh
set -eu

# only call build.sh if the file was not built yet
if [ ! -f ./build/release/dos_rs.exe ]; then
    ./build.sh release
fi
# add dos_rs.exe, dosbox.conf and CWSDPMI.EXE into new zip file
cd bundle
cp ../build/release/dos_rs.exe ./dos_rs.exe
rm -f dos_rs.zip
zip -q dos_rs.zip dos_rs.exe .jsdos/dosbox.conf CWSDPMI.EXE
rm dos_rs.exe

# move it as dos_rs.jsdos
cp dos_rs.zip ../dos_rs.jsdos
rm dos_rs.zip
echo "Created bundle dos_rs.jsdos"

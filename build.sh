#!/usr/bin/env sh
set -eu

elf2djgpp=elf2djgpp
target=debug
iset=${ISET:-i386}

if [ "${1-}" = "release" ]; then
    target="release"
fi

xflags=""
if [ "$target" = "release" ]; then
    xflags="--release"
fi

cargo build $xflags -Zbuild-std --target $iset-unknown-none-gnu.json

# Extract the object files from the ELF static library
mkdir -p build/$target/djgpp-lib
cd build/$target/djgpp-lib
rm -f *.o
llvm-ar x ../../../target/$iset-unknown-none-gnu/"$target"/libdos_rs.a

echo "Converting ELF objects to COFF-GO32..."
for f in *.o; do
    $elf2djgpp -q "$f" "$f.new"
    rm -f "$f"
    mv "$f.new" "$f"
done
# clean up the previous one
rm -f ../libdos_rs.a
llvm-ar cr ../libdos_rs.a *.o

echo "libdos_rs.a built"
echo "Building executable..."
i686-pc-msdosdjgpp-gcc -o ../dos_rs.exe ../libdos_rs.a
echo "build/$target/dos_rs.exe built"

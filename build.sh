#!/usr/bin/env sh
set -eu

CC=${CC:-i686-pc-msdosdjgpp-gcc}
AR=${AR:-ar}
ARCH=${ARCH:-i386}
elf2djgpp=elf2djgpp
target=debug

if [ "${1-}" = "release" ]; then
    target="release"
fi

RUST_XFLAGS=""
if [ "$target" = "release" ]; then
    RUST_XFLAGS="--release"
fi

cargo build $RUST_XFLAGS --target $ARCH-unknown-none-gnu.json

APPNAME="ferris"
LIBNAME="lib$APPNAME.a"

# Extract the object files from the ELF static library
mkdir -p build/$target/djgpp-lib
cd build/$target/djgpp-lib
rm -f *.o
${AR} x "../../../target/$ARCH-unknown-none-gnu/$target/$LIBNAME"

echo "Converting ELF objects to COFF-GO32..."
for f in *.o; do
    $elf2djgpp -q "$f" "$f.new"
    rm -f "$f"
    mv "$f.new" "$f"
done
# clean up the previous one
rm -f "../$LIBNAME"
${AR} cr "../$LIBNAME" *.o

echo "$LIBNAME built"
echo "Building executable..."

C_XFLAGS="-march=${ARCH}"

if [ "$target" = "release" ]; then
    C_XFLAGS="$C_XFLAGS -O2"
else
    C_XFLAGS="$C_XFLAGS -O0 -g"
fi

$CC $C_XFLAGS -o ../ferris.exe "../$LIBNAME"
echo "build/$target/ferris.exe built"

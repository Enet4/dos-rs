
# MS-DOS with Rust

[![ci](https://github.com/Enet4/dos-rs/actions/workflows/ci.yml/badge.svg)](https://github.com/Enet4/dos-rs/actions/workflows/ci.yml)
[![pages-build-deployment](https://github.com/Enet4/dos-rs/actions/workflows/pages/pages-build-deployment/badge.svg)](https://github.com/Enet4/dos-rs/actions/workflows/pages/pages-build-deployment)

This is an attempt at building a Rust project to target
MS-DOS in protected mode via [DJGPP](http://www.delorie.com/djgpp/),
supporting x86 CPUs architectures from i386 to i686.

To engage in the discussion to reach this goal, please check out [this GitHub issue](https://github.com/Serentty/rusty-dos/issues/3) and [this thread](https://groups.google.com/forum/#!msg/comp.os.msdos.djgpp/0l6wjO-oSM0/wucHtHpCAgAJ).
After many different attempts,
the one that seems the most promising right now is
the conversion of ELF .o objects into DJGPP COFF32 objects.

This project also contains a few preliminary modules that grant easier
access to DOS-specific capabilities, namely port I/O, calling interrupts,
and VGA graphics.

## Overview

In this repository you will find:

- [djgpp](djgpp): a low-level library for interfacing with the libc and DJGPP API
- [dos_x](dos_x): an experimental library to assist in the creation of DOS programs
- [examples/ferris](examples/ferris): an example program that shows a picture
- and instructions on how to make this all work.

## Status

While there are not many stability and performance guarantees at the moment,
the proofs of concept written so far appear to work as intended.
There is no `std` support,
but an allocator is available.

The development experience is also not as fluid as it could be.
The Rust program exports a C main function,
so it exists as a static C library.
The compiled objects need to be converted
before they are linked together using `i686-pc-msdosdjgpp-gcc`.

Known caveats:

- Be aware of soundness issues in the compilation of floating point arithmetic
  against targets without SSE2.
  <https://github.com/rust-lang/rust/issues/114479>
  The use of `f32` or `f64` may be unreliable,
  so test carefully.

## Requirements

- Build and install [`elf2dgpp`](https://github.com/cknave/elf2djgpp)
- A nightly Rust toolchain (defined in [rust-toolchain](rust-toolchain))
- The [DJGGP GCC toolchain](https://www.delorie.com/djgpp)
  (version 12.2.0 is known to work, but it should work with more versions as is).

## Building

```sh
./build.sh
# or
./build.sh release
```

Some variables in the script can be tuned to your liking.

## Running

Copy the resulting `dos_rs.exe` file into your DOS environment,
with [`CWSDPMI.EXE`](http://sandmann.dotster.com/cwsdpmi/) alongside it.
It should then be ready to run on a DOS machine, virtual machine, or emulator.

## Related

[Serentty/rusty-dos](https://github.com/Serentty/rusty-dos): a repository presenting a semi-successful attempt at compiling a Rust program running in 16-bit real mode DOS.

## License

Licensed under either of

- Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.

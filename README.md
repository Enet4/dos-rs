
# MS-DOS with Rust

This is an attempt at building a Rust project to target
MS-DOS in protected mode via [DJGPP](http://www.delorie.com/djgpp/),
supporting x86 CPUS from i386 to i686.

To engage in the discussion to reach this goal, please check out [this GitHub issue](https://github.com/Serentty/rusty-dos/issues/3) and [this thread](https://groups.google.com/forum/#!msg/comp.os.msdos.djgpp/0l6wjO-oSM0/wucHtHpCAgAJ).
After many different attempts,
the one that seems the most promising right now is
the conversion of ELF .o objects into DJGPP COFF32 objects.

This project also contains a few preliminary modules that grant easier
access to DOS-specific capabilities, namely port I/O, calling interrupts,
and VGA graphics.

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

Some variables in the script can be tweaked beforehand.

## Running

Copy the resulting `dos_rs.exe` file

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


# MS-DOS with Rust

This is an attempt at the construction of the Rust build target `i686-ps-msdos-djgpp`: a target for MS-DOS in protected mode via [DJGPP](http://www.delorie.com/djgpp/), supporting x86 CPUS from i386.

This is currently not functioning as intended, but publishing the current efforts in a repository makes it easier for others to reproduce and continue iterating on the concept.

To engage in the discussion to reach this goal, please check out [this GitHub issue](https://github.com/Serentty/rusty-dos/issues/3) and [this thread](https://groups.google.com/forum/#!msg/comp.os.msdos.djgpp/0l6wjO-oSM0/wucHtHpCAgAJ).

## Requirements

- A nightly Rust toolchain (defined in [rust-toolchain](rust-toolchain)).
- [Xargo](https://github.com/japaric/xargo): `cargo install xargo`
   - Although there are ways to build without it, xargo appears to work more consistently across toolchains.
- The DJGGP GCC toolchain.

## Building

```sh
RUST_TARGET_PATH=`pwd` xargo build
```

On Linux, you can run [`build.sh`](build.sh), which contains a few other useful commands, such as disassembling the EXE file and creating a list of symbols.

## Related

[Serentty/rusty-dos](https://github.com/Serentty/rusty-dos): a repository presenting a semi-successful attempt at compiling a Rust program running in 16-bit real mode DOS, 

## License

Licensed under either of

* Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.

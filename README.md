# CrabOS

a monolithic kernel written in rust to x86 architecture, with a userland shell.

`(\/) (°,,,,°) (\/)`

## Build & Run

1. install `rust` & `qemu-system-x86_64`
2. change you `rustc` channel to nightly
   ```bash
   rustup default nightly-x86_64-unknown-linux-gnu
   ```
   to check your `rustc` details run `rustc --version --verbose`
3. cargo install `bootimage`
   ```bash
   cargo install bootimage
   ```

4. add `rustc` component `llvm-tools-preview` and `rust-src`
   ```bash
   rustup component add rust-src --toolchain nightly-x86_64-unknown-linux-gnu
   rustup component add llvm-tools-preview
   ```
5. `cargo run`


## Debug the kernel

1. add the `-s -S` options to the qemu command.
2. load the binary with gdb `gdb /path/to/CrabOS`
3. run in gdb `target remote localhost:1234`
4. start debugging :)

## Feature

- [x] UART for exceptions and unit testing
- [x] VGA buffer
- [x] interrupts & exceptions
- [x] physical memory manager
- [x] virtual memory manager
- [x] kernel heap management
- [x] processes
- [x] syscall structure & userland
- [ ] file system
- [ ] shell and some commands
- [ ] maybe security stuff...

## Research

- [Memory Management](research/mm.md)
- [Interrupts](research/interrupts.md)
- [Miscellaneous](research/miscellaneous.md)
- [Rust](research/rust.md)

## Resources

[osdev](https://wiki.osdev.org/Main_Page)\
[intel's developer manual](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html)

Tutorials

- [Writing an OS in rust](https://os.phil-opp.com/)
- [digital whisper tutorial](https://www.digitalwhisper.co.il/files/Zines/0x1E/DW30-4-OsDev.pdf)

Repositories

- [byteOS](https://github.com/64/ByteOS)
- [MonkOS](https://github.com/beevik/MonkOS)
- [AlmeidaOS](https://github.com/PauloMigAlmeida/AlmeidaOS)
- [arbel-os](https://github.com/arbel03/os)

# CrabOS

a monolithic kernel written in rust to x86 architecture, with a userland shell.

`(\/) (°,,,,°) (\/)`

## Build & Run

`cargo run`

## Feature

- [ ] UART for exceptions and unit testing
- [ ] VGA buffer
- [ ] interrupts & exceptions
- [ ] physical memory manager
- [ ] virtual memory manager
- [ ] heap management **
- [ ] processes and scheduling
- [ ] syscall structure & userland
- [ ] file system
- [ ] shell and some commands
- [ ] maybe security stuff...

## Research

- [Memory Management](research/mm.md)
- [Interrupts](research/interrupts.md)
- [miscellaneous](research/miscellaneous.md)

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

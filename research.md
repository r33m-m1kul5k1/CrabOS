# OS x86

***

## Boot sequence

### MBR & VBR

the boot sector. containing the `bootloader` and the disk `partition table`. `VBR` is the loader of the kernel inside it's partition.
**NOTE**: both commands to update the partition table and load the `VBR` uses `BOIS INT 13h`.

### Protected Mode & Real Mode

`Real Mode` - the first mode all x86 processors starts with (16 bit) \
`Protected Mode` - enables IO protections and rings (supervisor and userland).

### The sequence

1. BOIS (POST)
2. MBR (Real mode to Protected mode switch), (Bootloader)
3. VBR
4. kernel init -> userland

## Kernel types

- monolithic, meaning all the services in the kernel (linux, and some windows versions).

- micro, most services runs as daemons in user space.

- hybrid, some services inside the kernel and some in user space.

- modular, minimal kernel with the option to add modules at any time when needed.

***

## Memory management

### Address space

virtual address space is converted to the physical address space through the `MMU`. The addresses on virtual address space are continuous and can only be accessed by the kernel. making the memory more secured and simple.

### Memory translation systems

#### Segmentation

##### real mode segmentation

The physical address is `(segment * 0x10) + offset`. segments can overlap. x86 segments are `CS, DS, SS, ES, FS, GS`.
The only way to change a `CS` register is by `Far jump/call`, `Far return/IRET`, `INT`.\

##### protected mode segmentation

The physical address is `GDT[selector] + offset`.

```TEXT
+-----------------------+---+-----+---+------+
| Base address (32 bit) | G | BD  |   |  A   |
+-----------------------+---+-----+---+------+
| Limit (20 bits)       | P | DPL | S | Type |
+-----------------------+---+-----+---+------+
```

for more info about the table click [this](https://en.wikipedia.org/wiki/Segment_descriptor#:~:text=In%20memory%20addressing%20for%20Intel,to%20in%20the%20logical%20address.)

**NOTE**: C isn't compatible with segmentation.

#### Paging

divide all memory to parts called pages, and using a table to map all physical addresses to virtual address.\
x86 implementation uses two tables, 4KiB each.

```TEXT
three level arch
cr3 -> page directory -> page table
```

the `MMU` converts the virutal address to physical address using the tables.

##### paging in long mode (64 bit)

now instead of three levels we can use up to 5 levels.

```TEXT
cr3 -> PML5 -> PML4 -> page directory -> page table
```

#### Page faults

occurs when

- asccessing unmapped virtual memory. (Pure)
- try to write to a read-only page.
- access a page without the premissions to.

### Virtual memory

is memory that is currently on disk. Rhis scheme enables a process to have "infinate" memory. By using a not so frequently used page. A good algorithm to decide which page to save to the swap file is `the working set`.\
**NOTE:** this method can be implemented on entire segments instead of pages.
***

## Rings

***

## [Exceptions & Interrupts](https://wiki.osdev.org/Exceptions)

***

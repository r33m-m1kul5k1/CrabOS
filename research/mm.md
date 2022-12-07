# [Memory management](https://wiki.osdev.org/Brendan%27s_Memory_Management_Guide)

## The architecture
`kmain`

1. frame distributer
2. virtual memory mapper
3. heap management with buddy
4. full physical memory allocation (buddy manager)


## Physical Memory Manager - pmm

**Function**: given a number of page frames return an array of addresses\
**Algorithm**: buddy

```Text
                0   4   8   12  16  20  24  28  32  36
                ###.#....#........#...###...########.... real memory pattern

buddy[0]--->  ###.#.xx.#xxxxxxxx#.xx###.xx########xxxx 5  free 4K , 5-byte bitmap
buddy[1]--->  # # # . # . x x . # . # # . # # # # x x  5  free 8K , 20-bits map
buddy[2]--->  #   #   #   .   #   #   #   #   #   .    2  free 16K, 10-bits map
buddy[3]--->  #       #       #       #       #        0  free 32K, 5-bits map
```

**Question** why not to use linked list?

## Virtual Memory Manager (paging) - vmm

**Functions**: map/unmap, create/destroy pages, by calling the pmm.

### pros

- memory resizing.
- efficient use of the ram, only what's needed can be loaded.
- security, execution prevention, permission checks.
- isolate programs memory.
- solves fragmentation with the pages scheme.

## Heap Manager (Flat memory model)

**Algorithm**: first fit & (bitmap | linked list)

other implementations can be found [here](https://wiki.osdev.org/User:Pancakes/SimpleHeapImplementation)

### process memory

a process memory can be divided to

```Text
+-------+
| stack | - constant*
+-------+
| heap  | - constant* 
+-------+
| text  | - constant
+-------+
```

*can be enlarged by a pure page fault.

**Address space**: a range of memory can be virtual or physical.

### DMA

a chip that transfer data from between memories.
it signals when it stops with an interrupt.
like a daemon for the CPU.

### Memory translation/mapping systems

#### Paging

divide all memory to parts called pages, and using a table to map all physical addresses to virtual address.\
x86 implementation uses two tables, 4KiB each. We use a multi-level page table to handle specific regions, each table points to multiple regions.

```TEXT
three level arch
cr3 -> page directory -> page table
```

the `MMU` converts the virtual address to physical address using the tables.

##### paging in long mode (64 bit)

now instead of three levels we can use up to 5 levels.

```TEXT
cr3 -> PML5 -> PML4 -> page directory -> page table
```

#### accessing the page table through virtual memory (recursive paging)

setup the bootloader to configure entry 511 to include the level 4 page table physical memory.\
To access page table 1 - the first 9 bits should be indexed to 511. to access page table 2 the first 18 bits should be 511_511.
and so on.

#### walking through the page tables

```rust
translate_virtual_address(virtual_address, physical_memory_offset)
{
    table_indexes = virtual_address.split();
    frame = Cr3::read(); // page map level 4

    for index in table_indexes {
        entry = frame[index]
        
        frame = match entry.frame() {
            Ok(frame) => frame,
            Err(FrameError::FrameNotPresent) => return None,
            Err(FrameError::HugeFrame) => panic!(“huge pages are not supported”),

        }

    }

    frame.start_address() + virtual_address.page_offset()
}
```

##### 64 bit entry format

```text
+--------+-----------------------+----------------------------------------------------------------------------------------------+
| Bit(s) |          Name         |                                            Meaning                                           |
+--------+-----------------------+----------------------------------------------------------------------------------------------+
| 0      | present               | the page is currently in memory                                                              |
+--------+-----------------------+----------------------------------------------------------------------------------------------+
| 1      | writable              | it’s allowed to write to this page                                                           |
+--------+-----------------------+----------------------------------------------------------------------------------------------+
| 2      | user accessible       | if not set, only kernel mode code can access this page                                       |
+--------+-----------------------+----------------------------------------------------------------------------------------------+
| 3      | write through caching | writes go directly to memory                                                                 |
+--------+-----------------------+----------------------------------------------------------------------------------------------+
| 4      | disable cache         | no cache is used for this page                                                               |
+--------+-----------------------+----------------------------------------------------------------------------------------------+
| 5      | accessed              | the CPU sets this bit when this page is used                                                 |
+--------+-----------------------+----------------------------------------------------------------------------------------------+
| 6      | dirty                 | the CPU sets this bit when a write to this page occurs                                       |
+--------+-----------------------+----------------------------------------------------------------------------------------------+
| 7      | huge page/null        | must be 0 in P1 and P4, creates a 1 GiB page in P3, creates a 2 MiB page in P2               |
+--------+-----------------------+----------------------------------------------------------------------------------------------+
| 8      | global                | page isn’t flushed from caches on address space switch (PGE bit of CR4 register must be set) |
+--------+-----------------------+----------------------------------------------------------------------------------------------+
| 9-11   | available             | can be used freely by the OS                                                                 |
+--------+-----------------------+----------------------------------------------------------------------------------------------+
| 12-51  | physical address      | the page aligned 52bit physical address of the frame or the next page table                  |
+--------+-----------------------+----------------------------------------------------------------------------------------------+
| 52-62  | available             | can be used freely by the OS                                                                 |
+--------+-----------------------+----------------------------------------------------------------------------------------------+
| 63     | no execute            | forbid executing code on this page (the NXE bit in the EFER register must be set)            |
+--------+-----------------------+----------------------------------------------------------------------------------------------+

```

#### Page faults

occurs when

- accessing unmapped virtual memory. (Pure)
- try to write to a read-only page.
- access a page without the permissions to.

### Virtual memory

is memory that is currently on disk. This scheme enables a process to have "infinite" memory. By using a not so frequently used page. A good algorithm to decide which page to save to the swap file is `the working set`.\
**NOTE:** this method can be implemented on entire segments instead of pages.

#### Segmentation

**NOTE:** we cannot use segmentation because it cannot handle fragmentation.

ldt - segments which are private to a specific program\
gdt - global segments.

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

***

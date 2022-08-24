# OS

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

## Memory management

### Address space

virtual address space is converted to the physical address space through the `MMU`. The addresses on virtual address space are continuous and can only be accessed by the kernel. making the memory more secured and simple.

### Memory translation systems

#### Segmentation

#### Paging

divide all memory to parts called pages, and using a table to map all physical addresses to virtual address.

### Virtual memory

is memory that is currently on disk. Rhis scheme enables a process to have "infinate" memory. By using a not so frequently used page. A good algorithm to decide which page to save to the swap file is `the working set`.

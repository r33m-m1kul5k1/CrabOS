# Miscellaneous

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
3. VBR - not really used anymore
4. kernel init -> userland

***

## Kernel types

- monolithic, meaning all the services in the kernel (linux, and some windows versions).

- micro, most services runs as daemons in user space.

- hybrid, some services inside the kernel and some in user space.

- modular, minimal kernel with the option to add modules at any time when needed.

***

## IO x86-64

### IO address space

a distinct address space from the physical memory address space. you can select the address space using the M/IO pin. Inside the IO address space is the input or output data for the specific port.

### MMIO

the IO address space is in the physical memory address space. This gives the user to use memory instructions such as MOV, AND OR TEST.
Note that this address space cannot be cached, using the MTRRs registers to mark this address space as uncachable.

***

## Rings

***

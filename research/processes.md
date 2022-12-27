# Process management

the flow of execution is as follows

1. spawning a process object with ID = 0
2. call exec on this process
3. for any new process run (fork & exec)

## Required System Calls

### exec

replaces the calling process's memory with a new memory image loaded from an ELF format file.

1. open the elf file from it
2. allocate pages for the process file
3. load the file into memory
4. update the stack with `argv`
5. context switch

### fork

creates a new process object, that points to the same executing process, with the only different is the: \
`pid, parent_id, child_id`

### exit

tells the kernel to release the process resources.

### write

### read

### open

### close


## [Elf format](https://en.wikipedia.org/wiki/Executable_and_Linkable_Format)

### ELF header

```rust
pub struct ElfHeader {
    magic: [u8; 4],
    class: u8, // 64 or 32 bit file
    endianness: u8, 
    version: u8,
    /*
    the interface between two binary programs, in this  
    case the OS and the user land program
    */
    os_abi: u8, 
    abi_version: u8,
    unused: [u8; 7],
    elf_type: u16,
    machine: u16,
    version2: u32,
    pub entry_point: u32,
    pub phoff: u32, // Program Header offset
    shoff: u32, // Section Header offset
    flags: u32,
    header_size: u16,
    pub phentsize: u16, // Program Header entry size
    pub phnum: u16, // Program Header entry count
    shentsize: u16, // Section Header entry size
    shnum: u16, // Section Header entry count
     // the index of the section names inside Section Header
    e_shstrndx: u16,
}
```

### Program Header Table

### Code Segment

### data Segment

### Sections' Names

### Section Header Table
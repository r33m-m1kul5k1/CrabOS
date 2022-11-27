# Shell

- execute other user-space binaries
- provide input output streams

## flow of binary execution

1. read a command from the user (name & arguments).
2. run `exec` syscall to run the user program
3. read input from the user to the binary
4. read output from the binary to the display

### requirements

- `exec`
- keyboard driver
- vga driver

#### A Basic shell

```C
void main(int argc, char* argv[]) // edit as appropriate for your kernel
{
    while (true) // you may want to provide a built-in "exit" command
    {
        char* command;
        int proc;
standard I/O streams
        output_prompt();               // display a prompt
        command = input_line();        // get a line of input, which will become the command to execute
        proc = process_start(command); // start a process from the command
        free(command);
 
        while (process_executing(proc))
        {
            if (input_line_waiting())
            {
                char* line;
                line = input_line();                 // read input from user
                process_send_input_line(proc, line); // send input to process
                free(line);
            }
            if (process_output_line_waiting(proc))
            {
                char* output;
                output = process_get_output_line(proc); // get output from process
                output_line(output);                    // write output to user
                free(output);
            }
        }
    }
}
```

### Bonus

- string editing
- working directory (fs)
- IO redirection & piping

## Required System Calls

### exec

replaces the calling process's memory with a new memory image loaded from an ELF format file.

1. open the elf file to read
2. allocate pages for the process file
3. load the file into memory
4. update the stack with `argv`

### fork

creates a new process object, by forking itself.

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

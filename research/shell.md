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
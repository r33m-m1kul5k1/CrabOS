## Booting Process

1. Power on.
2. CPU will move to BIOS (Basic Input Output System) in ROM (Read Only Memory).
3. BIOS will be executed (POST - Power On Self Test) : all the hardware will be tested.
This is a working condition to the hardware which is related to the CPU:
If the post is successful, then the hardware is working.
If the hardware is not working, the booting process will automatically stop.
4. BIOS will load MBR (Master Boot Record) to RAM (Random Access Memory).
5. MBR will load bootloader to RAM.
6. Bootloader will load OS to RAM.
7. ow, It starts executing the OS, and the control will move to the OS.

Hard Booting - Power on.
Soft Booting - Restart.

### [Booting Process](https://www.youtube.com/watch?v=bDsTwHIqs2g)
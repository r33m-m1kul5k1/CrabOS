# Interrupts

interrupts are signals from devices to the CPU, telling it to stop an do something else. \
The IDT is a table of 0xff entries each for an interrupt handler or interrupt service routine ISR.\
interrupt vector table = IDT in Real Mode.
**potential attacks**: idt hooking

## Types of interrupts

***

### [Exceptions](https://wiki.osdev.org/Exceptions)

Signals generated internally by the CPU.
Exceptions classified as:

- Faults - an error that can be dealt with.
- Traps - for debugging.
- Aborts - unrecoverable errors.

**all exceptions gate type is trap**
***

### IRQ (Hardware Interrupt)

IRQ can be classified as:

#### Pin-base IRQs

a physical connection between the device to an IRQ controller that communicates with the CPU.

#### MSI - Message Signaled Interrupts

To pass a signal first you need to configure the PCI.
Then passing a signal through the PCI to the APIC / IOAPIC.

#### PCI

a bus for attaching hardware devices to your motherboard. This bus gives the devices a configuration space.
The PCI supports interrupts signalling using the configuration and multiple buss lines.

#### PIC / APIC / IOAPIC  - interrupts controllers

The role of the PIC is to decide whether CPU will be interrupted or not. Moreover it converts the IRQ number to an interrupt vector (0-255) and distribute the interrupts to the targeted processes.
The interrupt controller structure:

```Text
  -----------
-|           |
-| Slave PIC |         ------------
-|           |--------|            |
  -----------         | Master PIC |-------CPU
----------------------|            |
----------------------|            |
                       ------------
```

### using IRQs

- enabling IRQs (sti instruction)
- signaling EOI using the PIC crate
- disabling IRQs can solve deadlocks
- hlt instruction halts the CPU till an interrupt occurs, more efficient than a endless loop
- pc-keyboard scancode crate
- port 0x60 -> for reading a scan code (you must)

***

### Software Interrupt

All interrupts that can be triggered by a software using the `int` instruction. (traps)
***

## CPU & Interrupts

Every instruction the CPU looks at the PIC pin to check if it has an interrupt. if it does then save the state on the stack, and handle the interrupt. If the Status register Interrupt Flag is off the CPU will ignore the PIC.

### Gate Types

Interrupt Gate -> return to the next instruction.\
Trap Gate -> return to the currently executed instruction. (interrupt could occur)

## Switching Stacks

when the stack hits the page guard, because the page guard isn't mapped to physical memory a page fault occurs. The CPU pushes to the interrupt stack frame which causes a second page fault. this causes a double fault which causes a triple fault for the same reason.

The solution is to use an Interrupt Stack Table, for a reserve stack. This reserved stack index is saved in the Entry Options on the IDT.

In order to store the IST we need the TSS for legacy reasons.

### TSS - Task State Segment

a table including pointers to:

1. Privilege Stack Table [0-2] when 0 is kernel stack
2. Interrupt Stack Table [0-6] total of 7 reserved stacks.
3. IO map Base Address

## GDT

we need to use this table for loading the TSS structure and switching between kernel and user space (exceptions).

```Rust
lazy_static! {
    pub static ref GDT: (GlobalDescriptorTable, Selectors) = {
        let mut gdt = GlobalDescriptorTable::new();

        let tss = gdt.add_entry(Descriptor::tss_segment(&TSS));
        let code = gdt.add_entry(Descriptor::kernel_code_segment());
        let data = gdt.add_entry(Descriptor::kernel_data_segment());
        let user_code = gdt.add_entry(Descriptor::user_code_segment());
        let user_data = gdt.add_entry(Descriptor::user_data_segment());

        (gdt, Selectors { tss, code, data, user_code, user_data })
    };
}
```

### Segment Selector

A reference to the Segment Descriptor that is saved into the appropriate segment register.

### Segment Descriptor

the segment entry in the GDT.


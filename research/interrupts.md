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
- Traps - not necessarily an error (breakpoint).
- Aborts - unrecoverable errors.
  
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

***

### Software Interrupt

These interrupts are used to call the kernel attention.
Mostly used for system calls.
***

## CPU & Interrupts

Every instruction the CPU looks at the PIC pin to check if it has an interrupt. if it does then save the state on the stack, and handle the interrupt. If the Status register Interrupt Flag is off the CPU will ignore the PIC.

### Kernel code for handling keyboard interrupt

```Text
push eax    ;; make sure you don't damage current state
in al,60h   ;; read information from the keyboard
 
mov al,20h
out 20h,al  ;; acknowledge the interrupt to the PIC
pop eax     ;; restore state
iret        ;; return to code executed before.
```

### ISR - interrupt service routines

to handle software interrupts:

```asm
/* filename: isr_wrapper.s 
isr_wrapper is a function pointer in the IDT
*/
.globl   isr_wrapper
.align   4
 
isr_wrapper:
    pushad
    cld /* C code following the sysV ABI requires DF to be clear on function entry */
    call interrupt_handler
    popad
    iret
```

***

### implementation structure

IdtEntry - not using segments -> intel's manual & tutorial.

idt initialize with the ISRs function pointers.

ISRs - specific

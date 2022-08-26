# Analyzing the kernel

## ByteOS

### init memory - 

in file : iso / boot / grub / grub.cfg

set timeout=0
set default=0

menuentry "Byte OS" {
    multiboot2 /boot/byteos.elf
    boot
}

in file : kernel / mm / vmm.c

extern struct page_table p4_table; // Initial kernel p4 table
struct mmu_info kernel_mmu;

__attribute__((aligned(PAGE_SIZE))) const uint8_t zero_page[PAGE_SIZE] = { 0 };

void vmm_init(void)
{
	kernel_mmu.p4 = phys_to_kern((physaddr_t)&p4_table);
	kernel_mmu.areas = NULL;
	klog("vmm", "Kernel P4 at %p\n", kernel_mmu.p4);
//	vmm_dump_tables();
}

# psudo code

init kernel
init page_table

void init_memory() 
{
    kernel._page_table = page table

}





### process memory creation -

### page faults - 

## MonkOS

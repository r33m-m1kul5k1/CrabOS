# Analyzing the kernel

## ByteOS

### init memory - 

# psudo code

```C
init kernel
init page_table

void init_memory() 
{
    kernel._page_table = page_table.get_physical_address() // TODO: how does it get the physicall address

}
```


### process memory creation -


# psudo code
```C
void create_proccess() {

    Task task 
    task.init_cpu_run_queue()
    Task forked_task = fork_task() // create the proccess + memory allocate and resources
    forked_task.start() // start the proccess - adds it to the run queue
    wait_for_scheduler() // wait for permission
    lapic_timer_enable() // Enables the lapic timer with interrupts every 10ms
    if(forked_task._is_resceduled == false)
    {
        schedule_and_run()
    }
}
```

### page faults - 


# psudo code : 

```	c
/*
Error - data type which contains:
1.  error code and interrupt number for exceptions
2. syscall number for syscalls
3. interrupt number otherwise
4. interrupt stack frame 
etc.. all unsigned long long int
*/
void handlePageFault(Error error) 
{
	ptr* faulting_address;
	// The fault was likely due to an access in kernel space, so give up
	if(faulting_address.used_kernel_space() == true)
	{
		kernel_panic() // 
	}
	// If interrupts were enabled, we are safe to enable them again
	else if(error.interrupt_was_unabled() == true)
	{
		enable()
	}
	else if(current_task.is_active() == true)
	{
		VirtualAddress vadd = error.get_virtual_addr()

		write_spin_lock(current_process->mmu->page_table); // locks the process until finish use the resource

		Adderss addr = vmm_get_virtual_memory_addr()
		PageTable pt = vmm_get_page_table(current->mmu, addr);
		bool done = cow_handle_write(pt, addr); // Returns true if the write is allowed and should be retried

		write_spin_unlock(current_process->mmu->page_table);

		if (done)
			return;
	}
	else 
	{
		kill_process()
	}


}
```
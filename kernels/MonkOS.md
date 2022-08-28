# Analyze

## initialization

```C
void mm_init()
{
    order_memory_blocks_from_BOIS();
    malloc_region(
        pa(kernel_start) - TEXT_OFFSET - STACK_SIZE, 
        round_up_po2(pa(kernel_start) - TEXT_OFFSET)
    )

    setup_buddy_allocator();
     
    
}
```


## stack

## heap

### allocation

## code segment

## page faults

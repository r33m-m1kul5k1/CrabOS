cargo build
qemu-system-x86_64 -device isa-debug-exit,iobase=0xf4,iosize=0x04 \
 -chardev stdio,id=char0,mux=on,logfile=history.log,signal=off \
  -serial chardev:char0 -mon chardev=char0 \
  -drive format=raw,file=target/x86_64/debug/bootimage-CrabOS.bin
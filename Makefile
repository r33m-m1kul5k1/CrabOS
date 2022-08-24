default:
	cargo bootimage
	qemu-system-x86_64 -nographic -drive format=raw,file=target/x86_64/debug/bootimage-rust_os.bin

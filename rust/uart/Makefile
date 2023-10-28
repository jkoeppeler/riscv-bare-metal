qemu: 
	qemu-system-riscv64 -machine virt -s -S -nographic -kernel target/riscv64imac-unknown-none-elf/debug/hello_world  -bios none

gdb: 
	riscv64-unknown-elf-gdb target/riscv64imac-unknown-none-elf/debug/hello_world

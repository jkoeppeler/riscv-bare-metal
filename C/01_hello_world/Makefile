hello: hello.o link.lds start.o
	riscv64-unknown-elf-ld -T link.lds -o hello hello.o start.o

hello.o: hello.c
	riscv64-unknown-elf-gcc -g -c hello.c -o hello.o

start.o: start.s
	riscv64-unknown-elf-as -o start.o start.s

clean:
	rm hello.o hello start.o


qemu: $(prog)
	qemu-system-riscv64 -machine virt -s -S -nographic -kernel $(prog) -bios none

gdb: 
	riscv64-unknown-elf-gdb $(prog)

# Setup
Start docker
```
./start_docker.sh
```

# Uart example
In the first terminal window, connect to the docker container.
Go to
```
cd ~/code/rust/uart
```

Build the binary
```
cargo build
```
Run the kernel
```
make qemu
```
You won't see any output because qemu is waiting for gdb to connect (-S)

To connect with gdb, open a second terminal window, and connect to the container
```
cd ~/code/rust/uart
```

```
make gdb
```

In the gdb console type `c` to start the execution and you will see that 'A's are printed in the first console.



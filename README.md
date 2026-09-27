# my-rtos (Cortex-M Task Scheduler) 

A small priority based task scheduler built for ARM Cortex-M3 using the thumbv7m-none-eabi target written to learn more about ARM architecture and the PendSV handler for context switching using inline assembly.

## Target

- 'thumbv7m-none-eabi'
  ```
  rustup target add thumbv7m-none-eabi
  ```
- QEMU machine: 'lm3s6965evb'

## Dependencies

- [`cortex-m`](https://crates.io/crates/cortex-m)
- [`cortex-m-rt`](https://crates.io/crates/cortex-m-rt)
- [`cortex-m-semihosting`](https://crates.io/crates/cortex-m-semihosting)
- [`panic-semihosting`](https://crates.io/crates/panic-semihosting)
- `qemu-system-arm` (for running/debugging)
- `gdb-multiarch` or `arm-none-eabi-gdb` (for debugging)

## Running

To be able to run directly with ``` cargo run ``` you need to have '.cargo/config.toml' configured like this:
```
[target.thumbv7m-none-eabi]
runner = "qemu-system-arm -cpu cortex-m3 -machine lm3s6965evb -nographic -semihosting-config enable=on -kernel"
rustflags = ["-C", "link-arg=-Tlink.x"]

[build]
target = "thumbv7m-none-eabi"
```

## Debugging

In the '/debug' directory you need to use the script: ```./qemu-debug.sh ../target/thumbv7m-none-eabi/debug/my-rtos```

And in another terminal: 
```
gdb ../target/thumbv7m-none-eabi/debug/my-rtos
(gdb) target remote :1234
``` 

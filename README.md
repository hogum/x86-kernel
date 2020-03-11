## x86-kernel
A minimal x86 kernel

### Setup
- Install [qemu](https://www.qemu.org/download/)
- Run **.deps.sh** for cargo deps `sh .deps.sh`
- Load the bootimage binary

```shell
    qemu-system-x86_64 -drive format=raw,file=target/.target/debug/bootimage-x86-kernel.bin 
```



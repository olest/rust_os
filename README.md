# rust_os

A small operating system in Rust (based on https://os.phil-opp.com/)

## How to build

`cargo bootimage`

`qemu-system-x86_64 -drive format=raw,file=./target/x86_64-rust_os/debug/bootimage-rust_os.bin`

## References

* [Bare Metal Rust](http://www.randomhacks.net/bare-metal-rust/)
* [OSdev.org: Rust](https://wiki.osdev.org/Rust)


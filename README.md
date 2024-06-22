# Hello World AArch64 Bare Metal Program
This repository contains a simple "Hello World" program designed to run on AArch64 bare metal environments. The program outputs a simple greeting message directly to the console when executed.

## Building the Project

To build the project, you must use the `--release` flag with `cargo build`. The debug build is currently not supported.

## Known Issues

Not work using debug build

## Running the Program with QEMU

To run the program, use the following command with QEMU. This will emulate an AArch64 machine using the Cortex-A72 CPU and run the program in a headless mode.

```
qemu-system-aarch64 -machine virt -cpu cortex-a72 -nographic -kernel target/aarch64-unknown-none/release/hello
```
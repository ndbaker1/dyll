# Slibbery - Shared Library Stub Generator

**Slibbery** (a pun on "shared lib foolery") is a tool that generates drop-in replacement shared libraries (.so files) for Linux that can intercept and mock function calls while forwarding others to the original library.

## Installation

```bash
cargo build --release
```

## Usage

Generate a stub library for any shared library:

```bash
./target/release/slibbery /path/to/libexample.so ./output_dir
cd output_dir
cargo build --release
LD_PRELOAD=./target/release/libmock_lib.so ./your_program
```

## Testing

```bash
make e2e
```
#!/bin/bash

set -eou pipefail

echo "=== E2E Test Suite for Shared Library Stub Generator ==="

# Clean up previous runs
echo "Cleaning up previous test artifacts..."
rm -f libtest.so
rm -f test_program_dlopen test_program_preload

# Create and compile test library
echo "Creating and compiling test shared library..."
gcc -shared -fPIC -g -o libtest.so test_lib.c

# Generate stub library
echo "Generating stub library..."
cargo run --release -- --output-dir mock/src --lib-path libtest.so header test_lib.h

# Build the stub library
echo "Building stub library..."
cd mock
SHARED_LIBRARY_PATH=../../libtest.so cargo build --release
cd ..

# Compile and run test programs
echo "Compiling test programs..."
gcc -o test_program_dlopen test_program_dlopen.c -ldl
gcc -o test_program_preload test_program_preload.c -lc -Wl,--unresolved-symbols=ignore-all

echo "Running test program with dlopen..."
./test_program_dlopen mock/target/release/libmock.so

echo "=== Test completed successfully! ==="

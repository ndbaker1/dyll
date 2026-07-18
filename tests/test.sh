#!/bin/bash

set -eou pipefail

echo "=== E2E Test Suite for Shared Library Stub Generator ==="

# Create and compile test library
echo "Creating and compiling test shared library..."
gcc -shared -fPIC -g -o ./output/libtest.so ../examples/program/test_lib.c

# Compile and run test programs
echo "Compiling test programs..."
gcc -o ./output/test_program_dlopen ../examples/program/test_program_dlopen.c -ldl
gcc -o ./output/test_program_preload ../examples/program/test_program_preload.c -lc -Wl,--unresolved-symbols=ignore-all

test_root=$(pwd)

folder=../examples/mock-rust

# Generate stub library
echo "Generating stub library..."
cargo run --release -- --output-path ${folder}/src/dyll.rs --lib-path libtest.so header ../examples/program/test_lib.h

# Build the stub library
echo "Building stub library..."
pushd ${folder}
SHARED_LIBRARY_PATH=${test_root}/libtest.so cargo build --release
mv target/release/libmock.so ${test_root}/output/libmock.so
popd

echo "Running test program with dlopen..."
${test_root}/output/test_program_dlopen ${test_root}/output/libmock.so

echo "=== Test completed successfully! ==="

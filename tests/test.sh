#!/bin/bash

set -eou pipefail

echo "=== E2E Test Suite for Shared Library Stub Generator ==="

artifacts_root=$(pwd)/artifacts
mock_root=$(pwd)/../examples/mock-rust
test_program_root=$(pwd)/../examples/program

# Create and compile test library
echo "Creating and compiling test shared library..."
mkdir -p ${artifacts_root}
gcc -shared -fPIC -g -o ${artifacts_root}/libtest.so ${test_program_root}/test_lib.c

# Compile and run test programs
echo "Compiling test programs..."
gcc -o ${artifacts_root}/test_program_dlopen ${test_program_root}/test_program_dlopen.c -ldl
gcc -o ${artifacts_root}/test_program_preload ${test_program_root}/test_program_preload.c -lc -Wl,--unresolved-symbols=ignore-all

# Generate stub library
echo "Generating stub library..."
cargo run --release -- --output-path ${mock_root}/src/dyll.rs header ${test_program_root}/test_lib.h

# Build the stub library
echo "Building stub library..."
pushd ${mock_root}
SHARED_LIBRARY_PATH=${artifacts_root}/libtest.so cargo build --release
mv target/release/libmock.so ${artifacts_root}/libmock.so
popd

echo "Running test program with dlopen..."
${artifacts_root}/test_program_dlopen ${artifacts_root}/libmock.so

echo "=== Test completed successfully! ==="

FROM rust AS builder

WORKDIR /workdir

# TODO: you'll need to adjust these on the builder to target the host.
# 
# downloads the nvidia management library .so file and uses it to build the nvml
# interception library.
RUN wget https://developer.download.nvidia.com/compute/nvidia-driver/redist/nvidia_driver/linux-x86_64/nvidia_driver-linux-x86_64-590.48.01-archive.tar.xz
RUN xz -d nvidia_driver-linux-x86_64-590.48.01-archive.tar.xz
RUN tar xvf nvidia_driver-linux-x86_64-590.48.01-archive.tar \
        nvidia_driver-linux-x86_64-590.48.01-archive/lib/libnvidia-ml.so.590.48.01 \
        --strip-components=2

# only need the source code.
COPY src src
COPY Cargo.* .

# run a debug build using the downloaded shared lib.
RUN SHARED_LIBRARY_PATH=/workdir/libnvidia-ml.so.590.48.01 cargo build

FROM alpine AS runner
# copy built rust shared lib into default workdir under the name of the nvidia
# management library.
COPY --from=builder /workdir/target/debug/libmock_lib.so libnvidia-ml.so.1

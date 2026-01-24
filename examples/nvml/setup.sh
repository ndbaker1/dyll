#!/usr/bin/env bash

# download nvidia-smi.
wget https://developer.download.nvidia.com/compute/nvidia-driver/redist/nvidia_driver/linux-x86_64/nvidia_driver-linux-x86_64-590.48.01-archive.tar.xz
xz -d nvidia_driver-linux-x86_64-590.48.01-archive.tar.xz
tar xvf nvidia_driver-linux-x86_64-590.48.01-archive.tar \
        nvidia_driver-linux-x86_64-590.48.01-archive/sbin/nvidia-smi \
        --strip-components=2
rm nvidia_driver-linux-x86_64-590.48.01-archive.tar

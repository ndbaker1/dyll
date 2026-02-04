#!/usr/bin/env bash

# the host can run using a local binary like nvidia-smi.
docker run --rm -it -e LD_LIBRARY_PATH=. libnvidia-ml-so:latest $@

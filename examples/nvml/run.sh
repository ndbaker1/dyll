#!/usr/bin/env bash

# the host can run using a local binary like nvidia-smi.
docker run --rm --entrypoint cat libnvidia-ml-so:latest libnvidia-ml.so.1 > libnvidia-ml.so.1 && $@

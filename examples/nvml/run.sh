#!/usr/bin/env bash

 docker build . -t libnvidia-ml-so && docker run --rm --entrypoint cat libnvidia-ml-so:latest libnvidia-ml.so.1 > libnvidia-ml.so.1 && $@

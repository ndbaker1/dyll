CONTAINER ?= docker

.PHONY: all clean test e2e docker

all:
	cargo build --release

clean:
	cargo clean
	rm -f tests/output/

test:
	cargo test

test-e2e:
	cd tests && ./test.sh

containerized:
	$(CONTAINER) run --rm -it -v "$$(pwd):/src" -w /src $$(docker build -q . -f Dockerfile.dev) $(WHAT)

CONTAINER ?= docker

.DEFAULT_GOAL := containerized

.PHONY: containerized
containerized:
	$(CONTAINER) run --rm -it -v "$(PWD):/app" -w /app $$(docker build -q . -f Dockerfile.dev) $(WHAT)

.PHONY: build
build:
	cargo build --release

.PHONY: clean
clean:
	cargo clean
	rm -f tests/output/

.PHONY: test
test:
	cargo test

.PHONY: test-e2e
test-e2e:
	cd tests && ./test.sh


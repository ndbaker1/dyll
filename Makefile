CONTAINER ?= docker

DEV_IMAGE ?= dyll-dev

.DEFAULT_GOAL := containerized

.PHONY: dev-image
dev-image:
	$(CONTAINER) build -t $(DEV_IMAGE) . -f Dockerfile.dev

.PHONY: containerized
containerized: dev-image
	$(CONTAINER) run --rm -it -v "$(PWD):/app" -w /app $(DEV_IMAGE) $(WHAT)

.PHONY: build
build:
	cargo build --release

.PHONY: clean
clean:
	cargo clean
	rm -f tests/artifacts/

.PHONY: test
test:
	cargo test

.PHONY: test-e2e
test-e2e:
	cd tests && ./test.sh


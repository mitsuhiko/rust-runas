.PHONY: build
build:
	@cargo build

.PHONY: test
test:
	@cargo test

.PHONY: docs
docs:
	@cargo doc --no-deps

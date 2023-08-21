build:
	cargo build

tests/gen:
	cd test && ./gen_suite.sh

.PHONY: test
test: build
	bats test/suite

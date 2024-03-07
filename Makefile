# Makefile

PHONY := all

all: test doc
	@echo "make all"

PHONY += build
build:
	@cargo build

PHONY += doc
doc:
	@cargo doc --open

PHONY += test
test:
	@cargo test

PHONY += bench
bench:
	@cargo bench 

.PHONY: $(PHONY)

# Makefile

PHONY := all

all:
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

.PHONY: $(PHONY)

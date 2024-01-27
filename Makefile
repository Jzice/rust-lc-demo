# Makefile

PHONY = all

all:
	@echo "make all"

PHONY += doc
doc:
	@cargo doc

PHONY += test
test:
	@cargo test

.phony: $(PHONY)

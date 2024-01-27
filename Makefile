
PHONY = all
all:

PHONY += test
test:
	@cargo test

.phony: $(PHONY)

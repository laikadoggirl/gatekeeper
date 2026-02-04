# Simple Makefile for gatekeeper
# Targets:
#   make build        - Build debug workspace
#   make release      - Build release CLI
#   make test         - Run workspace tests (locked)
#   make fmt          - Check formatting
#   make clippy       - Run clippy (deny warnings)
#   make install      - Install CLI via cargo install --path
#   make uninstall    - Uninstall CLI
#   make completions  - Generate completions into ./dist/completions
#   make man          - Generate man page into ./dist/man
#   make clean        - Clean target directory

CARGO ?= cargo
PACKAGE := gatekeeper
DIST := dist

.PHONY: build release test fmt clippy install uninstall completions man clean

build:
	$(CARGO) build --workspace

release:
	$(CARGO) build --release -p $(PACKAGE)

test:
	$(CARGO) test --workspace --locked

fmt:
	$(CARGO) fmt --check

clippy:
	$(CARGO) clippy --workspace --all-targets -- -D warnings

install:
	$(CARGO) install --path crates/$(PACKAGE) --locked

uninstall:
	$(CARGO) uninstall $(PACKAGE) || true

completions:
	mkdir -p $(DIST)/completions
	./target/release/$(PACKAGE) completions bash > $(DIST)/completions/$(PACKAGE).bash || (echo "Build release first: make release" && false)
	./target/release/$(PACKAGE) completions zsh > $(DIST)/completions/_$(PACKAGE)
	./target/release/$(PACKAGE) completions fish > $(DIST)/completions/$(PACKAGE).fish

man:
	mkdir -p $(DIST)/man
	./target/release/$(PACKAGE) manpage > $(DIST)/man/$(PACKAGE).1 || (echo "Build release first: make release" && false)

clean:
	$(CARGO) clean

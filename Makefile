.PHONY: build install uninstall clean help

build:
	cargo build --release

install:
	./install.sh install

uninstall:
	./install.sh uninstall

clean:
	cargo clean

help:
	@echo "Available targets:"
	@echo "  build      - Build the project in release mode"
	@echo "  install    - Build and install the 'lc' command to /usr/local/bin"
	@echo "  uninstall  - Remove the installed 'lc' command"
	@echo "  clean      - Clean build artifacts"
	@echo "  help       - Show this help message"

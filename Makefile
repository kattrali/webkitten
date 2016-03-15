PROJECT=webkitten
# Target installation directory
DESTDIR := /usr/local
# Command to create a directory, default is BSD-style install
INSTALLDIRCMD := install -d
# Command to install file
INSTALLCMD := install -C
# Libraries required to build
LIBS=webkit2gtk-4.0 gtk+-3.0
# Linking flags for required libraries. The spaces are added for cargo compat.
CFLAGS:= $(subst -L/,-L /,$(subst  -l, -l ,$(shell pkg-config --libs $(LIBS))))
# Cargo build manager
CARGO=CFLAGS='$(CFLAGS)' cargo

SRC_FILES=src/main.rs build.rs Cargo.toml
DEV_FILE=target/debug/$(PROJECT)
PROD_FILE=target/release/$(PROJECT)
INSTALL_FILE=$(DESTDIR)/bin/$(PROJECT)

all: build

$(DEV_FILE): $(SRC_FILES)
	@$(CARGO) build

$(PROD_FILE): $(SRC_FILES)
	@$(CARGO) build --release

.PHONY: build

build: $(DEV_FILE) ## Build the webkitten binary

release: $(PROD_FILE) ## Build the webkitten binary in release mode

install: $(PROD_FILE) ## Install webkitten into $DESTDIR/bin
	@$(INSTALLDIRCMD) $(DESTDIR)/bin
	@$(INSTALLCMD) $(PROD_FILE) $(INSTALL_FILE)

uninstall: ## Remove webkitten from $DESTDIR/bin
	@rm $(INSTALL_FILE)

clean: ## Clean the build environment
	@$(CARGO) clean

run: ## Run webkitten in development mode
	@$(CARGO) run

test: ## Run the webkitten test suite
	@$(CARGO) test

help: ## Show help text
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'

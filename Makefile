# Target installation directory
DESTDIR := /usr/local
# Subdirectory within $(DESTDIR) for installing the binaries
DESTBIN := $(DESTDIR)/bin
# Command to create a directory, default is BSD-style install
INSTALLDIRCMD := install -d
# Command to install file
INSTALLCMD := install -C

ifeq ($(shell uname),Darwin)
PROJECT=webkitten-cocoa
CARGO=cd $(PROJECT) && cargo
CARGO_TEST=cargo
else
PROJECT=webkitten-gtk
# Libraries required to build
LIBS=webkit2gtk-4.0 gtk+-3.0
# Linking flags for required libraries. The spaces are added for cargo compat.
CFLAGS:= $(subst -L/,-L /,$(subst  -l, -l ,$(shell pkg-config --libs $(LIBS))))
# Cargo build manager
CARGO=cd $(PROJECT) && CFLAGS='$(CFLAGS)' cargo
CARGO_TEST=CFLAGS='$(CFLAGS)' cargo
endif

SRC_FILES=$(shell ls src/*.rs $(PROJECT)/src/{**/,}*.rs) build.rs Cargo.toml
DEV_FILE=$(PROJECT)/target/debug/$(PROJECT)
PROD_FILE=$(PROJECT)/target/release/$(PROJECT)
INSTALL_FILE=$(DESTBIN)/$(PROJECT)
COCOA_APP=webkitten-cocoa/build/Release/Webkitten.app
COCOA_SRC=webkitten-cocoa/app/main.swift

all: build

$(DEV_FILE): $(SRC_FILES)
	@$(CARGO) build

$(PROD_FILE): $(SRC_FILES)
	@$(CARGO) build --release

$(COCOA_APP): $(PROD_FILE) $(COCOA_SRC)
	@cd webkitten-cocoa && xcodebuild
	@echo Generated $(COCOA_APP)

# Create the target directory for installing tool binaries if it does not
# exist
$(DESTBIN):
	@$(INSTALLDIRCMD) $(DESTBIN)

.PHONY: build

apidoc: ## Generate API documentation and open in the default browser
	@$(CARGO) doc --no-deps --open

doc: ## Generate user/development documentation
	$(MAKE) -C docs html

build: $(DEV_FILE) ## Build the webkitten binary

cocoa: $(COCOA_APP) ## Build the Cocoa application wrapper

cocoa-clean:
	@rm -r $(COCOA_APP)

release: $(PROD_FILE) ## Build the webkitten binary in release mode

install: $(PROD_FILE) ## Install webkitten into $DESTDIR/bin
	@$(INSTALLDIRCMD) $(DESTDIR)/bin
	@$(INSTALLCMD) $(PROD_FILE) $(INSTALL_FILE)

uninstall: ## Remove webkitten from $DESTDIR/bin
	@rm $(INSTALL_FILE)

clean: ## Clean the build environment
	@$(CARGO) clean

run: ## Run webkitten in development mode
	@RUST_LOG='info' $(CARGO) run

test: ## Run the webkitten test suite
	@$(CARGO_TEST) test
	@$(CARGO) test

help: ## Show help text
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'

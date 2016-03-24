# webkitten

A WebKit2-based hacker's browser, inspired by luakit, written in Rust

## Goals

* Human-readable configuration in plain text files
* [Pass](https://www.passwordstore.org) integration
* SQLite bookmark storage
* Scriptable custom commands in Lua
* Private browsing mode(s)
* [WebKit content blocking](https://webkit.org/blog/3476/content-blockers-first-look)
* Command bar autocompletion
* Per-site userscripts and css
* Customizable keybindings

## Usage

`webkitten` depends on:

* Gtk+ 3.0
* WebKit2Gtk+ 4.0

Both must be present to link and run.

## Development

### Dependencies

* make
* pkg-config
* (BSD) install, or be comfortable editing the Makefile environment
* Rust 1.5+, with [Cargo](http://doc.crates.io)

### Building

To build, run `make`.

### Installation

To install, run `make install`.

For all other commands, try `make help`.

### Resources

#### Gtk

* [Gtk+3](https://developer.gnome.org/gtk3/stable)
* [GObject](https://developer.gnome.org/gobject/stable)
* [WebKit2Gtk+](http://webkitgtk.org/reference/webkit2gtk/stable)
* [GtkSourceView](https://developer.gnome.org/gtksourceview/stable)
* [Gtk rust bindings](http://gtk-rs.org/docs)

#### WebKit

* [WebKit Wiki](http://trac.webkit.org/wiki)
* [WebKit Bugzilla](https://bugs.webkit.org)

#### Rust

* [The book](https://doc.rust-lang.org/stable/book)
* [Standard Library](http://doc.rust-lang.org/std)
* [libc](https://doc.rust-lang.org/stable/libc/index.html)
* [FFI guide](https://doc.rust-lang.org/book/ffi.html)
* [Cargo build script guide](http://doc.crates.io/build-script.html)
* [The Rustonomicon: Guide to Advanced/`unsafe` Rust](https://doc.rust-lang.org/nightly/nomicon)

#### Build System

* [Make reference](http://www.freebsd.org/doc/en/books/developers-handbook/tools-make.html)
* [pkg-config](https://www.freedesktop.org/wiki/Software/pkg-config)

#### Documentation

* [Sphinx](http://www.sphinx-doc.org/en/stable)
* [reStructuredText Specification](http://docutils.sourceforge.net/docs/ref/rst/restructuredtext.html)

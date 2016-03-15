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

* GTK+ 3.0
* WebKit2GTK 4.0

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

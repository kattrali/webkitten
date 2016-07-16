# webkitten

A hacker's browser toolkit inspired by luakit and written in Rust

## Goals

* Human-readable configuration in plain text files
* Scriptable custom commands in Lua
* SQLite bookmark storage
* Private browsing mode(s)
* [WebKit content blocking](https://webkit.org/blog/3476/content-blockers-first-look)
* Autocompletion interface for commands
* Per-site userscripts and css
* Customizable keybindings

## Usage

Webkitten depends on **Lua 5.2**, which must be present to link and run. Using
the webkitten toolkit requires implementing the `ui` module and starting the
application with an implementation of `ui::ApplicationUI`:

```rust
// Create runtime configuration
let run_config = RunConfiguration { path: path_to_config_toml, start_pages: vec![] };
// Create engine
let engine = Engine::new(run_config);
// Create UI
let mut ui = UI::new(engine);
ui.run();
```

Existing implementations:

* [Webkitten-cocoa](webkitten-cocoa/)
* [Webkitten-gtk](webkitten-gtk/) (incomplete)

While named "webkitten", new UI bindings do not necessarily need to be
WebKit-based. An implementation using Servo could be of particular interest.

## Development

### Dependencies

* make
* pkg-config
* (BSD) install, or be comfortable editing the Makefile environment
* Rust 1.5+, with [Cargo](http://doc.crates.io)

### Building

To build, run `make`. To run the reference implementations, use `make run`.

### Installation

To install, run `make install`.

For all other commands, try `make help`.

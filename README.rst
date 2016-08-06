webkitten
=========

Webkitten is a command-driven web browser toolkit inspired by luakit_ and Vim_.

Webkitten allows you to:

- Browse the web (nearly) pointing device-free
- Run custom scripts for browser interaction on demand or triggered by events
- Edit human-readable configuration files
- Assign keybindings to your custom scripts
- Alter web pages with custom CSS and JavaScript
- Create custom browsing modes based on the sites you visit
- Customize your own content blocking

Running a reference implementation
----------------------------------

In addition to the tooling, Webkitten includes two reference implementations of
the browser interface:

- webkitten-gtk_: A WebKit2 GTK+3 implementation of Webkitten with Lua
  scripting
- webkitten-cocoa_: A Cocoa WebKit implementation of Webkitten with Lua
  scripting

Use ``make run`` to run the default implementation for your platform, and see
the `User Guide`_ and the contrib_ directory for commands to kick start your
configuration. Use ``make install`` to install the binary into your ``PATH``.

Building your own browser
-------------------------

Using the webkitten toolkit requires implementing the `ui` module and starting
the application with an implementation of `ui::ApplicationUI`:

.. code-block:: rust

    // Create runtime configuration
    let run_config = RunConfiguration {
      path: path_to_config_toml,
      start_pages: vec!["https://example.com"]
    };

    // Create engine
    let engine = Engine::new(run_config);

    // Create UI
    let mut ui = MyCustomUI::new(engine);

    // Go go go
    ui.run();

Then the UI should notify the ``EventHandler`` when events occur, such as
pressing the Return key in the command bar or web content failing to load.
Provided this contract is met, the scripting engine can automate interactions
with the UI, making it easy to customize.

While named "webkitten", new UI bindings do not necessarily need to be
WebKit-based, though the bindings were designed with WebKit in mind.

Development
-----------

Webkitten is largely written in Rust and uses Cargo_ for dependency management.
Questions, suggestions, and patches welcome - see the `Contribution Guide`_ for
more information.

Building
~~~~~~~~

To build, run `make`. To run the reference implementations, use `make run`.

For all other commands, try `make help`.

.. _luakit: https://mason-larobina.github.io/luakit
.. _Vim: https://www.vim.org
.. _webkitten-gtk: webkitten-gtk
.. _webkitten-cocoa: webkitten-cocoa
.. _`User Guide`: https://delisa.me/webkitten
.. _contrib: contrib/scripts
.. _Cargo: https://docs.crates.io
.. _`Contribution Guide`: CONTRIBUTING.rst

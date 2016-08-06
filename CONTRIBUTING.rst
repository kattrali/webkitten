Contribution Guide
==================

The Webkitten project uses a pull request flow for accepting patches. To get
started:

1. Fork the project
2. Create a new branch for each logically grouped set of changes. In general,
   patches with multiple unrelated changes will probably not be accepted.
3. Open a pull request with a description of the changes made and the rationale

For bug fixes and adding new commands to the ``contrib/`` directory, no
additional steps are necessary. However, for new features, best results can be
achieved by first opening an issue to ensure its a direction which aligns with
project goals.

Building Webkitten
------------------

Webkitten depends on Rust 1.5+ with Cargo. Once installed, run ``make`` to
download the Rust dependencies, build the library, and build default reference
implementation.

Testing
-------

Use ``make test`` to run the library and default implementation tests. For
other commands, see ``make help``.

Development resources
---------------------

Core
****

Rust
~~~~

* The book (https://doc.rust-lang.org/stable/book)
* Standard Library (http://doc.rust-lang.org/std)
* libc (https://doc.rust-lang.org/stable/libc/index.html)
* FFI guide (https://doc.rust-lang.org/book/ffi.html)
* Cargo build script guide (http://doc.crates.io/build-script.html)
* The Rustonomicon: Guide to Advanced/``unsafe`` Rust (https://doc.rust-lang.org/nightly/nomicon)

Build System
~~~~~~~~~~~~

* Make reference (http://www.freebsd.org/doc/en/books/developers-handbook/tools-make.html)
* pkg-config (https://www.freedesktop.org/wiki/Software/pkg-config)

Documentation
~~~~~~~~~~~~~

The HTML documentation is compiled with Sphinx_ and reStructuredText_. Use ``pip
install -r requirements.txt`` to install the dependencies and ``make doc`` to
generate the files.

Reference implementations
*************************

GTK+
~~~~

* Gtk+3 (https://developer.gnome.org/gtk3/stable)
* GObject (https://developer.gnome.org/gobject/stable)
* WebKit2Gtk+ (http://webkitgtk.org/reference/webkit2gtk/stable)
* GtkSourceView (https://developer.gnome.org/gtksourceview/stable)
* Gtk rust bindings (http://gtk-rs.org/docs)

Cocoa
~~~~~

* AppKit Framework Reference
  (https://developer.apple.com/library/mac/documentation/Cocoa/Reference/ApplicationKit/ObjC_classic/index.html)

WebKit
~~~~~~

* WebKit Wiki (http://trac.webkit.org/wiki)
* WebKit Bugzilla (https://bugs.webkit.org)


.. _Sphinx: http://www.sphinx-doc.org/en/stable
.. _reStructuredText: http://docutils.sourceforge.net/docs/ref/rst/restructuredtext.html

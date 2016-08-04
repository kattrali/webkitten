Webkitten for GTK+
==================

A GTK+3 implementation of the webkitten interface, integrated with the Lua
scripting engine.

Installation
------------

Webkitten depends on:

- `GTK+3`_
- `WebKit2GTK+`_
- GtkSourceView_

From source
~~~~~~~~~~~

Development dependencies:

- make
- pkg-config
- (BSD) install, or be comfortable editing the Makefile environment
- Rust 1.5+, with Cargo (http://doc.crates.io)

Once the dependencies are satisfied, run ``make gtk`` from the root of the
repository to generate the ``webkitten-gtk`` binary.

Homebrew
~~~~~~~~

Install ``webkitten-gtk`` from tap using

.. code-block:: bash

    brew install kattrali/formulae/webkitten-gtk


Options
-------

The webkitten-cocoa binary can be used to open a URI (or several) directly and
specify a custom configuration path.

.. code-block:: text

   Usage: webkitten-gtk [options] [FILE]

   Options:
       -c, --config PATH   Use this configuration path
       -h, --help          Print this help text

Customization
-------------

At this point, you can run ``webkitten-gtk``, but it is a blank slate without
customized configuration or registered commands. Its a matter of personal
preference how you want webkitten to work for you and what commands and keyboard
shortcuts (among other things) should be available.

1. View the `configuration options reference`_ and customize your configuration
   as needed
2. Select commands from webkitten/contrib_ which may be useful to you and
   install them into your `command search path`_.
3. Write any other commands you need to make your browser perfect.

Next steps
----------

.. toctree::
   :maxdepth: 2

   configuration-options
   scripting-with-lua

.. _command search path: configuration-options.html#command-search-path
.. _configuration options reference: configuration-options.html
.. _contrib: https://github.com/kattrali/webkitten/tree/master/contrib
.. _`GTK+3`: http://www.gtk.org
.. _`WebKit2GTK+`: https://webkitgtk.org
.. _GtkSourceView: https://developer.gnome.org/gtksourceview/unstable/pt01.html

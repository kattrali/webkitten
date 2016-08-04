Webkitten for Cocoa
===================

A native Cocoa implementation of the webkitten interface, integrated with the
Lua scripting engine.

Installation
------------

Webkitten depends on:

- OS X 10.10+

From source
~~~~~~~~~~~

Development dependencies:

- make
- (BSD) install, or be comfortable editing the Makefile environment
- Rust 1.5+, with Cargo (http://doc.crates.io)

Once the dependencies are satisfied, run ``make cocoa`` from the root of the
repository to generate the ``Webkitten.app`` bundle.

Homebrew
~~~~~~~~

Install ``webkitten-cocoa`` from tap using

.. code-block:: bash

   brew cask install kattrali/formulae/webkitten-cocoa

Options
-------

The webkitten-cocoa binary can be used to open a URI (or several) directly and
specify a custom configuration path. However, the default mode for running the
app is through an app bundle, obviating these options.

.. code-block:: text

   Usage: webkitten-cocoa [options] [FILE] [FILE ...]

   Options:
       -c, --config PATH   Use this configuration path
       -h, --help          Print this help text

Customization
-------------

At this point, you can run Webkitten.app, but it is a blank slate without
customized configuration or registered commands. Its a matter of personal
preference how you want webkitten to work for you and what commands and keyboard
shortcuts (among other things) should be available.

1. View the `configuration options reference`_ and customize your configuration
   as needed. The default configuration file location is
   ``$HOME/.config/webkitten/config.toml``.
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

Webkitten browser toolkit
=========================

Webkitten is a command-driven web browser toolkit inspired by luakit_ and Vim_.

Webkitten allows you to:

- Browse the web (nearly) pointing device-free
- Run custom scripts for browser interaction on demand or triggered by events
- Edit human-readable configuration files
- Assign keybindings to your custom scripts
- Alter web pages with custom CSS and JavaScript
- Create custom browsing modes based on the sites you visit
- Customize your own content blocking

In addition to the tooling, Webkitten includes two reference implementation of
a the browser interface:

- webkitten-cocoa_: A Cocoa WebKit implementation of Webkitten with Lua
  scripting
- webkitten-gtk_: [WIP] A WebKit2 GTK+3 implementation of Webkitten with Lua
  scripting

User Guide
----------

.. toctree::
   :maxdepth: 2

   user-guide/webkitten-cocoa
   user-guide/webkitten-gtk
   user-guide/configuration-options
   user-guide/scripting-with-lua

Developer Guide
---------------

.. toctree::
   :maxdepth: 2

   dev-guide/building
   dev-guide/gui-binding
   dev-guide/script-binding
   dev-guide/contributing

.. _luakit: https://mason-larobina.github.io/luakit
.. _Vim: https://www.vim.org
.. _webkitten-gtk: user-guide/webkitten-gtk.html
.. _webkitten-cocoa: user-guide/webkitten-cocoa.html

Scripting with Lua
==================

Webkitten includes a Lua 5.2 scripting engine for executing scripts based on
input to the command bar and other event triggers.

When a command is run in Webkitten, it is matched to a file found within a
directory specified by the configuration option ``commands.search-paths``. For
example, if ``reading-mode on`` is entered in the command bar, Webkitten checks
the command search paths for a file named ``reading-mode.lua``, and if a match
is found, the file contents are evaluated and the ``run()`` method is called,
provided the script is not in some way malformed.

Each command runs in a new Lua runtime, so there is no interaction between
different commands.

Event triggers
--------------

The available entry points for running a command within Webkitten. Implement
any of the following methods to run when an event is triggered. Note that some
events are only triggered if a configuration prerequisite is met and that the
``description()`` method is required.

.. glossary::

     ``complete_commands()``
       Provides completions to command arguments. The scope of the function
       includes a ``prefix`` variable which returns the full text the user has
       entered, as well as a table of each individual argument as
       ``arguments``. Returns a comma-delimited list of items as a string, or
       an empty string if no results were found.

       .. code-block:: lua

          function complete_command()
            return "open,close,save"
          end

     ``description()``
       A summary of the script's purpose, intended for GUI bindings to display
       in autocompletion and help text. Returns a string. (required)

       .. code-block:: lua

          function description()
            return "An example script documenting all hooks"
          end

     ``on_fail_uri()``
       Invoked when a URI fails to load. The current scope includes a
       ``webview_index`` and ``window_index`` indicating which view is active,
       as well as ``requested_uri`` indicating what URI was requested.

       This hook is only invoked if the command name is included in the
       configuration option ``commands.on-fail-uri``.

       .. code-block:: lua

          function on_fail_uri()
            log_debug("Failed to load " .. requested_uri)
          end

     ``on_load_uri()``
       Invoked when a URI is loaded in a webview. The current scope includes a
       ``webview_index`` and ``window_index`` indicating which view is active,
       as well as ``requested_uri`` indicating what URI was requested.

       This hook is only invoked if the command name is included in the
       configuration option ``commands.on-load-uri``.

       .. code-block:: lua

          function on_load_uri()
            local uri = webview_uri(window_index, webview_index)
            log_debug(string.format("Just loaded %s", uri))
          end

     ``on_request_uri()``
       Invoked when a URI will be loaded in a webview. The current scope
       includes a ``webview_index`` and ``window_index`` indicating which view
       is active, as well as ``requested_uri`` indicating what URI was
       requested.

       This hook is only invoked if the command name is included in the
       configuration option ``commands.on-request-uri``.

       .. code-block:: lua

          function on_request_uri()
            log_debug("Requested to load " .. requested_uri)
          end

     ``run()``
       The default hook, invoked when the user presses Return in the command
       bar. The scope of the function includes an ``arguments`` variable, which
       is a table of the space-delimited arguments which were passed with the
       function. Returns a boolean indicating whether to clear the bar text.

       .. code-block:: lua

          function run()
            log_info("Running the example script")
            return true
          end

Constants
---------

The Lua scripting engine provides a few constant values to make it easier to
validate values or check configuration options.

.. glossary::

     ``CONFIG_FILE_PATH``
       The path to the configuration file being used by Webkitten. It can be
       used as a convenience to do configuration value lookup.

     ``NOT_FOUND``
       This is a possible value returned from the ``focused_window_index`` or
       ``focused_webview_index`` methods respectively, if there is no window or
       webview to correspond to the provided values. For example, if there are
       no windows open, ``focused_window_index()`` returns ``NOT_FOUND``.

Provided methods
----------------

In addition to the `Lua standard libraries`_, the following global methods are
provided for use within event triggers.

.. glossary::

     ``add_styles(window_index, webview_index, css)``
       Inject CSS into a webview

       .. code-block:: lua

          function run()
            local window_index = focused_window_index()
            local webview_index = focused_webview_index(window_index)
            add_styles(window_index, webview_index, [[
              body { background-color: red; }
            ]])
          end

     ``close_webview(window_index, webview_index)``
       Close a webview at a given index

     ``close_window(window_index)``
       Close a window with a given index

     ``command_field_text(window_index)``
       The text in the command bar of a window at a given index

     ``command_field_visible(window_index)``
       Return ``true`` if in the command bar of a window at a given index is
       visible

     ``copy(string)``
       Copy text to the native clipboard

     ``find(int, int, string)``
       Find and highlight text in a webview

     ``focus_commandbar_in_window(window_index)``
       Assign keyboard focus to the command field area of the window at a given
       index

     ``focus_webview(window_index, webview_index)``
       Show a webview at a given index and assign keyboard focus to it

     ``focus_webview_in_window(window_index)``
       Assign keyboard focus to the webview area of the window at a given index

     ``focused_webview_index(int)``
       Returns the index of the focused webview in a window at a given index or
       ``NOT_FOUND``

     ``focused_window_index()``
       Returns the index of the focused window or ``NOT_FOUND``

     ``go_back(window_index, webview_index)``
       Returns to the previously loaded resource (if any) in a webview at a
       given index

     ``go_forward(window_index, webview_index)``
       Loads the next resource (if any) in a webview at a given index

     ``hide_find(window_index, webview_index)``
       Hide any GUI elements or highlighting relating to finding text onscreen

     ``hide_window(window_index)``
       Hide a window at a given index

     ``load_uri(window_index, webview_index, string)``
       Load a resource from a URI in a webview at a given index

     ``log_debug(message)``
       Write text to the application log with a severity level of debug

     ``log_info(message)``
       Write text to the application log with a severity level of info

     ``lookup_bool(string)``
       Gets a bool value from the user's configuration file using the argument
       as a key

     ``lookup_string(config_path, key)``
       Gets a string value from the user's configuration file using the
       argument as a key

     ``lookup_strings(config_path, key)``
       Gets a table of strings from the user's configuration file using the
       argument as a key

     ``open_webview(window_index, uri)``
       Open a new webview in a window at a given index and load the URI

     ``open_window(uri)``
       Open a new window and load the URI

     ``reload_webview(int, int, bool)``
       Reload a webview, optionally skipping content filters

     ``resize_window(window_index, width, height)``
       Resize a window to the specified width and height

     ``run_javascript(window_index, webview_index, script)``
       Run JavaScript source code in the webview at a given index

     ``set_command_field_text(window_index, text)``
       Change the command field text in a window at a given index

     ``set_command_field_visible(window_index, is_visible)``
       Change the command field visibility in a window at a given index

     ``set_window_title(window_index, title)``
       Change the title in a window at a given index

     ``show_window(window_index)``
       Show a previously hidden window by index

     ``webview_count(window_index)``
       Returns the number of webviews contained in a window at a given index or
       zero if a window does not exist for that index

     ``webview_title(window_index, webview_index)``
       The title of the web content in a webview at a given index

     ``webview_uri(window_index, webview_index)``
       The URI of the loaded resource in a webview at a given index

     ``window_count()``
       The number of windows currently open

     ``window_title(window_index)``
       The title of the window at a given index or empty string if the index
       does not correspond to a window

.. _`Lua standard libraries`: https://www.lua.org/manual/5.2/manual.html#6

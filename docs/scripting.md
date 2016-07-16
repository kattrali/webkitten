# Scripting for Webkitten

Webkitten includes a built-in scripting engine for executing scripts based on
input to the command bar as well as event triggers.

## Hooks

When a command is run in Webkitten, it is matched to a script found via path
specified by the configuration option `commands.search-paths`. The script is
evaluated and the correct method is invoked if the script is not in some way
malformed. Only the `description()` hook is required to be implemented. The
following methods are available as hooks which can be invoked by different
events.

### `complete_commands()`

Provides completions to command arguments. The scope of the function includes a
`prefix` variable which returns the full text the user has entered, as well as a
table of each individual argument as `arguments`. Returns a comma-delimited list
of items as a string.

```lua
function complete_command()
  return "open,close,save"
end
```

### `description()`

A summary of the script's purpose, intended for UI bindings to display in
autocompletion and help text. Returns a string. (required)

```lua
function description()
  return "An example script documenting all hooks"
end
```

### `on_fail_uri()`

Invoked when a URI fails to load. The current scope includes a `webview_index`
and `window_index` indicating which view is active, as well as `requested_uri`
indicating what URI was requested.

This hook is only invoked if the command name is included in the configuration
option `commands.on-fail-uri`.

```lua
function on_fail_uri()
  log_debug("Failed to load " .. requested_uri)
end
```

### `on_load_uri()`

Invoked when a URI is loaded in a webview. The current scope includes a
`webview_index` and `window_index` indicating which view is active.

This hook is only invoked if the command name is included in the configuration
option `commands.on-load-uri`.

```lua
function on_load_uri()
  log_debug(string.format("Just loaded %s", uri(window_index, webview_index)))
end
```

### `on_request_uri()`

Invoked when a URI will be loaded in a webview. The current scope includes a
`webview_index` and `window_index` indicating which view is active, as well
as `requested_uri` indicating what URI was requested.

This hook is only invoked if the command name is included in the
configuration option `commands.on-request-uri`.

```lua
function on_request_uri()
  log_debug("Requested to load " .. requested_uri)
end
```

### `run()`

The default hook, invoked when the user presses Return in the command bar. The
scope of the function includes an `arguments` variable, which is a table of the
space-delimited arguments which were passed with the function. Returns a boolean
indicating whether to clear the bar text.

```lua
function run()
  log_info("Running the example script")
  return true
end
```

## Method reference

In addition to the
[Lua standard libraries](http://lua-users.org/wiki/StandardLibraries), the
following global methods are provided for use within hooks.

### Configuration

* `lookup_strings(string)`: Gets a table of strings from the user's
  configuration file using the argument as a key
* `lookup_string(string)`: Gets a string value from the user's configuration
  file using the argument as a key
* `lookup_bool(string)`: Gets a bool value from the user's configuration file
  using the argument as a key

### Logging

* `log_debug(string)`: Write text to the application log with a severity level
  of debug
* `log_info(string)`: Write text to the application log with a severity level of
  info

### Native functions

* `copy(string)`: Copy text to the native clipboard

### Window management

* `close_window(int)`
* `command_field_text(int)`
* `focused_webview_index(int)`
* `focused_window_index()`
* `hide_window(int)`
* `open_window(string)`
* `resize_window(int, int, int)`
* `set_command_field_text(int, string)`
* `show_window(int)`
* `window_count()`

### Webview management

* `add_styles(int, int, string)`: inject CSS from a string into a webview
* `close_webview(int, int)`
* `find(int, int, string)`: Find and highlight text in a webview
* `focus_webview(int, int)`
* `focused_webview_index(int)`
* `go_back(int, int)`
* `go_forward(int, int)`
* `hide_find(int, int)`
* `load_uri(int, int, string)`
* `open_webview(int, string)`
* `run_javascript(int, int, string)`
* `webview_count(int)`
* `webview_title(int, int)`
* `webview_uri(int, int)`

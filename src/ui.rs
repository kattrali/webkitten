use std::collections::HashMap;
use url::Url;

use keybinding;


pub trait ApplicationUI: Sized {

    /// Create a new UI
    fn new(engine: super::Engine) -> Option<Self>;

    /// UI event handler
    fn event_handler(&self) -> &super::Engine;

    /// Initialize all needed UI functions
    fn run(&self);

    /// Copy text to the system clipboard
    fn copy(&self, text: &str);


    /// The index of the focused window
    fn focused_window_index(&self) -> Option<u32>;

    /// Number of open windows
    fn window_count(&self) -> u32;

    /// Open a new window
    fn open_window(&self, uri: Option<&str>) -> u32;

    /// Close a window
    fn close_window(&self, index: u32);

    /// Focus window at index
    fn focus_window(&self, index: u32);

    /// Capture keyboard input in given area
    fn focus_window_area(&self, index: u32, area: WindowArea);

    /// Set window visibility
    fn toggle_window(&self, index: u32, visible: bool);

    /// Change the dimensions of a specified window
    fn resize_window(&self, window_index: u32, width: u32, height: u32);

    /// Text in the command bar of a specified window
    fn command_field_text(&self, window_index: u32) -> String;

    /// Set the text in the command bar of a specified window
    fn set_command_field_text(&self, window_index: u32, text: &str);

    /// Title of a specified window
    fn window_title(&self, window_index: u32) -> String;

    /// Set the title of a specified window
    fn set_window_title(&self, window_index: u32, title: &str);


    /// Index of the webview currently visible in a specified window
    fn focused_webview_index(&self, window_index: u32) -> Option<u32>;

    /// Number of webviews in a window
    fn webview_count(&self, window_index: u32) -> u32;

    /// Open a new webview in a specified window
    fn open_webview(&self, window_index: u32, uri: Option<&str>);

    /// Close a webview in a specified window
    fn close_webview(&self, window_index: u32, webview_index: u32);

    /// Focus a webview in a specified window, hiding the current webview
    fn focus_webview(&self, window_index: u32, webview_index: u32);

    /// Reload a webview in a specified window
    fn reload_webview(&self, window_index: u32, webview_index: u32, disable_filters: bool);

    /// Load a URI in a webview
    fn set_uri(&self, window_index: u32, webview_index: u32, uri: &str);

    /// Go back to the previously loaded resource in a webview
    fn go_back(&self, window_index: u32, webview_index: u32) -> bool;

    /// Go forward to the next loaded resource in a webview
    fn go_forward(&self, window_index: u32, webview_index: u32) -> bool;

    /// Get the currently loaded URI or empty string
    fn uri(&self, window_index: u32, webview_index: u32) -> String;

    /// Find a string within the selected web view
    fn find_string(&self, window_index: u32, webview_index: u32, query: &str);

    /// Hide results from a previous find invocation (if applicable)
    fn hide_find_results(&self, window_index: u32, webview_index: u32);

    /// Get the title of the currently loaded URI or empty string
    fn webview_title(&self, window_index: u32, webview_index: u32) -> String;

    /// Run a JavaScript snippet in a webview
    fn run_javascript(&self, window_index: u32, webview_index: u32, script: &str);

    /// Apply a stylesheet to a webview
    fn apply_styles(&self, window_index: u32, webview_index: u32, styles: &str);
}

pub enum WindowArea {
    CommandBar,
    WebView,
}

pub enum CommandError {
    /// No command matches the given text
    CommandNotFound,
    /// Command execution halted with an error
    ErrorDuringExecution,
    /// The provided arguments were invalid in the context of the given command
    InvalidArguments,
    /// There was no command text specified
    NoCommandSpecified,
}

pub struct CommandOutput {
    pub error: Option<CommandError>,
    pub message: Option<String>,
}

#[derive(Debug,Copy,Clone)]
pub enum URIEvent {
    Fail,
    Load,
    Request,
}

pub trait EventHandler {

    /// Handle a Return key press within the command bar
    fn execute_command<T: ApplicationUI>(&self, ui: &T, window_index: Option<u32>, text: &str)
        -> CommandOutput;

    /// Close the application
    fn close<T: ApplicationUI>(&self, ui: &T);

    /// Get available commands and/or arguments given a prefix
    fn command_completions<T: ApplicationUI>(&self, ui: &T, prefix: &str) -> Vec<String>;

    /// Handle a document load event in a webview.
    ///
    /// ## Events
    ///
    /// * `URIEvent::Request`: Invoke before document begins loading
    /// * `URIEvent::Load`: Invoke after document finishes loading but not
    ///   necessarily after subresources load
    /// * `URIEvent::Fail`: Invoke after a document fails to load
    fn on_uri_event<T: ApplicationUI>(&self,
                                      ui: &T,
                                      window_index: u32,
                                      webview_index: u32,
                                      uri: &str,
                                      event: URIEvent);

    /// Handle a request to open a URI in a new frame
    fn on_new_frame_request<T: ApplicationUI>(&self, ui: &T, window_index: u32, uri: &str);
}

pub trait BrowserConfiguration: Sized {

    /// Parse a string literal into a `BrowserConfiguration`
    fn parse(raw_input: &str) -> Option<Self>;

    /// The page opened with each new window or empty buffer based on
    /// `window.start-page`
    fn start_page(&self) -> Option<String> {
        self.lookup_str("window.start-page")
    }

    /// Whether to open a buffer in the focused window or a new window when
    /// requesting a new frame. Defaults to `false`, always opening a new
    /// window.
    fn new_frame_uses_focused_window(&self) -> bool {
        self.lookup_bool("new-frame.opens-in-focused-window")
            .unwrap_or(false)
    }

    /// The directory to replace instances of CONFIG_DIR in the configuration
    /// file
    fn config_dir(&self) -> Option<String> {
        self.lookup_raw_str("general.config-dir")
    }

    /// The name of a command resolving any matching alias in `commands.aliases`
    fn resolved_command_name(&self, name: &str) -> Option<String> {
        let command = self.lookup_str(&format!("commands.aliases.{}", name))
            .unwrap_or(String::from(name));
        if self.command_disabled(&command) { None } else { Some(command) }
    }

    /// Font to use in the command bar
    fn bar_font(&self) -> Option<(String, i64)> {
        if let Some(family) = self.lookup_str("general.bar-font.family") {
            if let Some(size) = self.lookup_integer("general.bar-font.size") {
                return Some((family, size));
            }
        }
        None
    }

    /// Find the command to automatically run for a given text prefix
    fn command_matching_prefix(&self, text: &str) -> Option<String> {
        if text.len() > 0 {
            let key = format!("commands.on-text-change.\"{}\"", &text[.. 1]);
            if let Some(script) = self.lookup_str(&key) {
                return Some(format!("{} {}", script, &text[1 ..]))
            }
        }
        None
    }

    /// Mapping of commands to keybindings by name to key and modifier mask
    fn command_keybindings(&self) -> HashMap<String, (char, usize)> {
        let mut table = HashMap::new();
        if let Some(bindings) = self.lookup_str_table("commands.keybindings") {
            for (command, binding) in bindings {
                match keybinding::parse(&binding) {
                    Ok((key, modifier)) => {
                        if let Some(_) = table.insert(command.to_owned(), (key, modifier)) {
                            warn!("Overriding keybinding ({}) for '{}'", binding, command);
                        }
                    },
                    Err(err) => error!("Failed to parse keybinding: {}", err)
                }
            }
        }
        table
    }

    /// Whether a command is disabled based on `commands.disabled`
    fn command_disabled(&self, name: &str) -> bool {
        if let Some(disabled) = self.lookup_str_vec("commands.disabled") {
            return disabled.contains(&String::from(name));
        }
        false
    }

    /// The path to the content filter used in buffers based on
    /// `general.content-filter`
    fn content_filter_path(&self) -> Option<String> {
        self.lookup_str("general.content-filter")
    }

    /// Whether to skip content filtering based on the site-specific option
    /// `sites."[HOST]".skip-content-filter`. Defaults to `false`.
    fn skip_content_filter(&self, uri: &str) -> bool {
        self.lookup_site_bool(uri, "general.skip-content-filter")
            .unwrap_or(false)
    }

    /// Whether to enable private browsing based on the global option
    /// `general.private-browsing` and site-specific option
    /// `sites."[HOST]".private-browsing`. Defaults to `false`.
    fn use_private_browsing(&self, uri: &str) -> bool {
        self.lookup_site_bool(uri, "general.private-browsing")
            .unwrap_or(false)
    }

    /// Whether to allow browser plugins to run in a buffer based on the global
    /// option `general.allow-plugins` and site-specific option
    /// `sites."[HOST]".allow-plugins`. Defaults to `false`.
    fn use_plugins(&self, uri: &str) -> bool {
        self.lookup_site_bool(uri, "general.allow-plugins")
            .unwrap_or(false)
    }

    /// Paths to search for command scripts using configuration option
    /// `command.search-paths`
    fn command_search_paths(&self) -> Vec<String> {
        self.lookup_str_vec("commands.search-paths").unwrap_or(vec![])
    }

    /// Command to run when no other commands are matched using configuration
    /// option `commands.default`
    fn default_command(&self) -> Option<String> {
        self.lookup_str("commands.default")
    }

    /// Commands triggered by a URI load event
    ///
    /// ## Events
    ///
    /// * `Load`: invokes all commands listed in `commands.on-load-uri`
    /// * `Request`: invokes all commands listed in `commands.on-request-uri`
    /// * `Fail`: invokes all commands listed in `commands.on-fail-uri`
    fn on_uri_event_commands(&self, event: URIEvent) -> Vec<String> {
        let key = match event {
            URIEvent::Load => "commands.on-load-uri",
            URIEvent::Request => "commands.on-request-uri",
            URIEvent::Fail => "commands.on-fail-uri",
        };
        self.lookup_str_vec(key).unwrap_or(vec![])
    }

    /// Look up the bool value of a configuration option matching key
    fn lookup_bool<'a>(&'a self, key: &'a str) -> Option<bool>;

    /// Look up the string value of a configuration option matching key,
    /// replacing string variables where possible
    fn lookup_str<'a>(&'a self, key: &'a str) -> Option<String>;

    /// Look up the integer value of a configuration option matching key
    fn lookup_integer<'a>(&'a self, key: &'a str) -> Option<i64>;

    /// Look up the string value of a configuration option without any
    /// substitutions
    fn lookup_raw_str<'a>(&'a self, key: &'a str) -> Option<String>;

    /// Look up the string vector value of a configuration option matching key
    fn lookup_str_vec(&self, key: &str) -> Option<Vec<String>>;

    /// Look up the string table value of a configuration option matching key
    fn lookup_str_table(&self, key: &str) -> Option<HashMap<String, String>>;

    /// Look up the bool value of a configuration option matching key
    /// formatted as `sites."[HOST]".[key]`, falling back to `[key]` if no
    /// match is found.
    fn lookup_site_bool<'a>(&'a self, uri: &str, key: &'a str) -> Option<bool> {
        construct_lookup_key(uri, key)
            .and_then(|key| self.lookup_bool(&key))
            .or(self.lookup_bool(&key))
    }

    /// Look up the string value of a configuration option matching key
    /// formatted as `sites."[HOST]".[key]`, falling back to `[key]` if no
    /// match is found.
    fn lookup_site_str<'a>(&'a self, uri: &str, key: &'a str) -> Option<String> {
        construct_lookup_key(uri, key)
            .and_then(|key| self.lookup_str(&key))
            .or(self.lookup_str(&key))
    }

    /// Look up the string vector value of a configuration option matching key
    /// formatted as `sites."[HOST]".[key]`, falling back to `[key]` if no
    /// match is found.
    fn lookup_site_str_vec<'a>(&'a self, uri: &str, key: &'a str) -> Option<Vec<String>> {
        construct_lookup_key(uri, key)
            .and_then(|key| self.lookup_str_vec(&key))
            .or(self.lookup_str_vec(&key))
    }
}

/// Determine the hostname component of a URI if possible and construct
/// the key for looking up an option
fn construct_lookup_key(uri: &str, key: &str) -> Option<String> {
    const URI_DELIMITER: &'static str = "://";
    const HTTP_PROTOCOL: &'static str = "http";
    let formatted_uri = if !uri.contains(URI_DELIMITER) {
        format!("{}{}{}", HTTP_PROTOCOL, URI_DELIMITER, uri)
    } else {
        String::from(uri)
    };
    if let Ok(url) = Url::parse(&formatted_uri) {
        if let Some(host) = url.host_str() {
            return Some(format!("sites.\"{}\".{}", host, key))
        }
    }
    warn!("Failed to parse URI: {}", uri);
    None
}

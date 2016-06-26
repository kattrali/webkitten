
pub trait ApplicationUI: Sized {

    /// Create a new UI
    fn new(engine: super::Engine) -> Option<Self>;

    /// UI event handler
    fn event_handler(&self) -> &super::Engine;

    /// Initialize all needed UI functions
    fn run(&self);

    /// Register content filtering rules to be applied to loaded resources
    fn register_content_filters(&self, identifier: &str, rules: &str);


    /// The index of the focused window
    fn focused_window_index(&self) -> u8;

    /// Number of open windows
    fn window_count(&self) -> u8;

    /// Open a new window
    fn open_window(&self, uri: Option<&str>);

    /// Close a window
    fn close_window(&self, index: u8);

    /// Focus window at index
    fn focus_window(&self, index: u8);

    /// Set window visibility
    fn toggle_window(&self, index: u8, visible: bool);

    /// Change the dimensions of a specified window
    fn resize_window(&self, window_index: u8, width: u32, height: u32);

    /// Text in the address bar of a specified window
    fn address_field_text(&self, window_index: u8) -> String;

    /// Set the text in the address bar of a specified window
    fn set_address_field_text(&self, window_index: u8, text: &str);

    /// Text in the command bar of a specified window
    fn command_field_text(&self, window_index: u8) -> String;

    /// Set the text in the command bar of a specified window
    fn set_command_field_text(&self, window_index: u8, text: &str);

    /// Title of a specified window
    fn window_title(&self, window_index: u8) -> String;

    /// Set the title of a specified window
    fn set_window_title(&self, window_index: u8, title: &str);


    /// Index of the webview currently visible in a specified window
    fn focused_webview_index(&self, window_index: u8) -> u8;

    /// Number of webviews in a window
    fn webview_count(&self, window_index: u8) -> u8;

    /// Open a new webview in a specified window
    fn open_webview(&self, window_index: u8, uri: &str);

    /// Close a webview in a specified window
    fn close_webview(&self, window_index: u8, webview_index: u8);

    /// Focus a webview in a specified window, hiding the current webview
    fn focus_webview(&self, window_index: u8, webview_index: u8);

    /// Load a URI in a webview
    fn set_uri(&self, window_index: u8, webview_index: u8, uri: &str);

    /// Go back to the previously loaded resource in a webview
    fn go_back(&self, window_index: u8, webview_index: u8) -> bool;

    /// Go forward to the next loaded resource in a webview
    fn go_forward(&self, window_index: u8, webview_index: u8) -> bool;

    /// Get the raw contents of the loaded resource in a webview
    fn raw_html(&self, window_index: u8, webview_index: u8, uri: &str) -> String;

    /// Get the currently loaded URI or empty string
    fn uri(&self, window_index: u8, webview_index: u8) -> String;

    /// Get the title of the currently loaded URI or empty string
    fn webview_title(&self, window_index: u8, webview_index: u8) -> String;

    /// Run a JavaScript snippet in a webview
    fn run_javascript(&self, window_index: u8, webview_index: u8, script: &str);

    /// Apply a stylesheet to a webview
    fn apply_styles(&self, window_index: u8, webview_index: u8, styles: &str);
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

pub enum AddressUpdateError {
    /// This address could not be resolved because it used an unsupported
    /// protocol
    ProtocolUnsupported,
    /// There was no address text specified
    NoAddressSpecified,
    /// Failed to resolve address
    ResolutionFailed,
}

pub struct CommandOutput {
    pub error: Option<CommandError>,
    pub message: Option<String>,
}

pub struct AddressUpdateOutput {
    pub error: Option<AddressUpdateError>,
    pub message: Option<String>,
}

pub trait EventHandler {

    /// Handle a Return key press within the command bar
    fn execute_command<T: ApplicationUI>(&self, ui: &T, window_index: u8, webview_index: u8, text: &str)
        -> CommandOutput;

    /// Handle a Return key press within the address bar
    fn update_address<T: ApplicationUI>(&self, ui: &T, window_index: u8, webview_index: u8, text: &str) -> AddressUpdateOutput;

    /// Close the application
    fn close<T: ApplicationUI>(&self, ui: &T);

    /// Get available commands and/or arguments given a prefix
    fn command_completions<T: ApplicationUI>(&self, ui: &T, prefix: &str) -> Vec<String>;

    /// Get available completions given addressable text, such as a URL
    /// fragment, page title, or bookmark
    fn address_completions<T: ApplicationUI>(&self, ui: &T, prefix: &str) -> Vec<String>;
}

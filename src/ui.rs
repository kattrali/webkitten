
pub trait ApplicationUI: Sized {

    /// Create a new UI
    fn new(event_handler: super::Engine) -> Option<Self>;

    /// UI event handler
    fn event_handler(&self) -> &super::Engine;

    /// Initialize all needed UI functions
    fn run(&mut self);

    /// Open a new window, returning the opened window
    fn open_window(&self, uri: &str);

    /// Window at index
    fn window<B: BrowserWindow>(&self, index: u8) -> Option<&B>;

    /// The index of the focused window
    fn focused_window_index(&self) -> u8;

    /// Focus window at index
    fn focus_window(&self, index: u8);

    /// Number of open windows
    fn window_count(&self) -> u8;
}

pub trait BrowserWindow {

    fn new() -> Self;

    fn show(&self);

    fn hide(&self);

    fn open_webview(&self, uri: String);

    fn close_webview(&self, index: u8);

    fn focus_webview(&self, index: u8);

    fn webview<W: WebView>(&self, index: u8) -> Option<&W>;

    fn resize(&self, width: u32, height: u32);

    fn address_field_text(&self) -> String;

    fn set_address_field_text(&self, text: String);

    fn command_field_text(&self) -> String;

    fn set_command_field_text(&self, text: String);

    fn focused_webview_index(&self) -> u8;
}

pub trait WebView {

    fn load_uri(&self, uri: &str);

    fn go_back(&self);

    fn go_forward(&self);

    fn focus(&self);

    fn raw_html(&self) -> String;

    fn uri(&self) -> String;

    fn title(&self) -> String;

    fn apply_javascript(&self, script: &str);

    fn apply_styles(&self, styles: &str);

    fn apply_content_filters(&self, identifier: &str, rules: &str);
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
    fn execute_command<T: ApplicationUI>(&self, ui: &T, window_index: u8, webview_index: u8, text: &str) -> CommandOutput;

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

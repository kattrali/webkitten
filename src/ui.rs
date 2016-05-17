
pub trait ApplicationUI: Sized {

    /// Create a new UI
    fn new() -> Option<Self>;
    //fn new<E: EventHandler>(event_handler: &E) -> Option<Self>;

    /// Initialize all needed UI functions
    fn run(&mut self);

    /// Open a new window, returning the opened window
    fn open_window<B: BrowserWindow>(&mut self, uri: String) -> &B;

    /// Window at index
    fn window<B: BrowserWindow>(&self, index: u8) -> Option<&B>;

    /// The index of the focused window
    fn focused_window_index() -> u8;

    /// Focus window at index
    fn focus_window(&mut self, index: u8);

    /// Number of open windows
    fn window_count(&self) -> u8;
}

pub trait BrowserWindow {

    fn show(&mut self);

    fn hide(&mut self);

    fn open_webview(&mut self, uri: String);

    fn close_webview(&mut self, index: u8);

    fn focus_webview(&mut self, index: u8);

    fn webview<W: WebView>(&self, index: u8) -> Option<&W>;

    fn resize(&mut self, width: u32, height: u32);

    fn address_field_text(&self) -> String;

    fn set_address_field_text(&self, text: String);

    fn command_field_text(&self) -> String;

    fn set_command_field_text(&self, text: String);

    fn focused_webview_index(&self) -> u8;
}

pub trait WebView {

    fn new() -> Self;

    fn load_uri(&self, uri: &'static str);

    fn go_back(&self);

    fn go_forward(&self);

    fn focus(&self);

    fn raw_html(&self) -> String;

    fn apply_javascript(&mut self, script: &str);

    fn apply_styles(&mut self, styles: &str);

    fn apply_content_blockers(&mut self, blockers: &str);
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
    error: Option<CommandError>,
    message: Option<String>,
}

pub struct AddressUpdateOutput {
    error: Option<AddressUpdateError>,
    message: Option<String>,
}

pub trait EventHandler {

    /// Handle a Return key press within the command bar
    fn execute_command(window_index: u8, webview_index: u8, text: &str) -> CommandOutput;

    /// Handle a Return key press within the address bar
    fn update_address(window_index: u8, webview_index: u8, text: &str) -> AddressUpdateOutput;

    /// Close the application
    fn close();

    /// Get available commands and/or arguments given a prefix
    fn command_completions(prefix: &str) -> Vec<String>;

    /// Get available completions given addressable text, such as a URL
    /// fragment, page title, or bookmark
    fn address_completions(prefix: &str) -> Vec<String>;
}

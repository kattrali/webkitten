
pub trait Application<B, E> {

    fn new<E: EventHandler>(config_path: &str, event_handler: E) -> Option<Self>;

    fn run(&mut self);

    fn add_window<B: BrowserWindow>(&mut self, uri: String) -> B;

    fn focused_window_index() -> u8;

    fn focus_window(index: u8);
}

pub trait BrowserWindow {

    fn new(title: &'static str) -> Self;

    fn show(&mut self);

    fn add_webview(&mut self, uri: String);

    fn set_size(&mut self, width: u32, height: u32);

    fn address_field_text() -> String;

    fn set_address_field_text(text: String);

    fn command_field_text() -> String;

    fn set_command_field_text(text: String);

    fn focused_webview_index() -> u8;

    fn focus_webview(index: u8);
}

pub trait WebView {

    fn new() -> Self;

    fn load_uri(&self, uri: &'static str);

    fn go_back(&self);

    fn go_forward(&self);

    fn focus(&self);

    fn raw_html(&self) -> String;

    fn apply_javascript(&mut self, script: &str);

    fn apply_css(&mut self, styles: &str);

    fn apply_content_blocker(&mut self, blockers: &str);
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
    fn handle_execute_command(window_index: u8, webview_index: u8, text: &str) -> CommandOutput;

    /// Handle a Return key press within the address bar
    fn handle_update_address(window_index: u8, webview_index: u8, text: &str) -> AddressUpdateOutput;
}

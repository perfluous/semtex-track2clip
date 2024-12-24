use arboard::Clipboard;

pub fn init_clipboard() -> Result<Clipboard, &'static str> {
    Clipboard::new().map_err(|_| "Error: Failed to access clipboard")
}

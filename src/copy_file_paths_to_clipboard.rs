use arboard::Clipboard;

pub fn set_clipboard_content(clipboard: &mut Clipboard, content: &str) -> Result<(), Box<dyn std::error::Error>> {
    clipboard.set_text(content.to_string())?;
    println!("Clipboard content: {}", content);  // Log what is being copied
    Ok(())
}

pub fn copy_file_paths_to_clipboard(file_contents: Vec<String>) -> Result<(), Box<dyn std::error::Error>> {
    let joined_contents = file_contents.join("\n\n");  // Join with double newlines for clarity between files
    set_clipboard_content(&mut Clipboard::new()?, &joined_contents)?;
    println!("Joined contents for clipboard: {}", joined_contents);  // Log the joined contents
    Ok(())
}

pub fn display_help() {
    println!("Usage: track2clip [OPTION] <repo_path>");
    println!("-h, --help             display this help and exit");
    println!("--include-hidden       include hidden files in the output");
    println!("--include-content      include file contents in the output");
    println!("--file-paths           copy specific file paths to the clipboard");
}

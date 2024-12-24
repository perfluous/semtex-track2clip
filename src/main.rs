use clap::{Arg, Command};
use std::process;

mod display_help;
mod validate_repo_path;
mod get_tracked_files;
mod process_files;
mod copy_file_paths_to_clipboard;

fn main() {
    let matches = Command::new("track2clip")
        .version("0.1.0")
        .author("Mark Monster")
        .about("Copy paths and contents of tracked files to clipboard")
        .arg(Arg::new("repo_path")
            .required(false)
            .help("Path to the git repository"))
        .arg(Arg::new("include-hidden")
            .long("include-hidden")
            .help("Include hidden files in the output"))
        .arg(Arg::new("paths-only")
            .long("paths-only")
            .help("Only copy file paths to the clipboard"))
        .arg(Arg::new("file_paths")
            .long("file-paths")
            .num_args(1..) // Accepts one or more arguments
            .help("Paths to specific files"))
        .get_matches();

    if matches.contains_id("help") {
        display_help::display_help();
        return;
    }

    let repo_path = matches.get_one::<String>("repo_path").map_or("", String::as_str);
    if repo_path.is_empty() {
        eprintln!("Error: Missing repo path argument");
        process::exit(1);
    }
    println!("Repo path: {}", repo_path);  // Log to verify the path

    let include_hidden = matches.contains_id("include-hidden");
    let paths_only = matches.contains_id("paths-only");  // New flag for paths only

    // Get specific file paths and copy them to the clipboard
    if let Some(file_paths) = matches.get_many::<String>("file_paths") {
        let file_paths: Vec<String> = file_paths.map(|s| s.to_string()).collect();
        println!("Copying file paths: {:?}", file_paths);  // Log the file paths

        if let Err(err) = copy_file_paths_to_clipboard::copy_file_paths_to_clipboard(file_paths) {
            eprintln!("Error: {}", err);
        } else {
            println!("File paths copied to clipboard successfully.");
        }
        return;
    }

    if let Err(err) = validate_repo_path::validate_repo_path(repo_path) {
        eprintln!("{}", err);
        process::exit(1);
    }

    let tracked_files = match get_tracked_files::get_tracked_files(repo_path, include_hidden) {
        Ok(files) => {
            println!("Tracked files: {:?}", files);  // Log the tracked files
            files
        },
        Err(err) => {
            eprintln!("{}", err);
            process::exit(1);
        }
    };

    // Default: Copy file contents unless --paths-only is specified
    if paths_only {
        println!("Copying file paths to clipboard...");
        if let Err(err) = copy_file_paths_to_clipboard::copy_file_paths_to_clipboard(tracked_files) {
            eprintln!("Error copying file paths to clipboard: {}", err);
        } else {
            println!("Tracked file paths copied to clipboard successfully.");
        }
    } else {
        println!("Copying file contents to clipboard...");
        let file_contents = process_files::process_files(repo_path, tracked_files.clone(), true);  // Always include content unless paths-only
        println!("File contents: {:?}", file_contents);  // Log the contents being copied

        if let Err(err) = copy_file_paths_to_clipboard::copy_file_paths_to_clipboard(file_contents) {
            eprintln!("Error copying file contents to clipboard: {}", err);
        } else {
            println!("File contents copied to clipboard successfully.");
        }
    }
}

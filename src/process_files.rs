use std::fs;
use std::path::Path;

pub fn process_files(repo_path: &str, tracked_files: Vec<String>, include_content: bool) -> Vec<String> {
    let mut file_contents = Vec::new();
    for file in tracked_files {
        let file_path = Path::new(repo_path).join(&file);
        println!("Processing file: {:?}", file_path);  // Log the file being processed

        if !file_path.exists() || !file_path.is_file() {
            println!("Error reading file: {:?}", file_path);
            continue;
        }

        if include_content {
            if let Ok(content) = fs::read_to_string(&file_path) {
                println!("File: {:?}", file_path);
                // Wrap the content in triple quotes with the file name
                let formatted_content = format!("{}\n```\n{}\n```", file_path.display(), content);
                file_contents.push(formatted_content);  // Collect the formatted content
            } else {
                println!("Error reading file content: {:?}", file_path);
            }
        }
    }
    println!("File contents collected: {:?}", file_contents);  // Log the collected contents
    file_contents
}

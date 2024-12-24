use git2::Repository;
use std::error::Error;

pub fn get_tracked_files(repo_path: &str, include_hidden: bool) -> Result<Vec<String>, Box<dyn Error>> {
    let repo = Repository::open(repo_path)?;
    let mut file_paths = Vec::new();

    for entry in repo.index()?.iter() {
        let file_name = String::from_utf8_lossy(&entry.path).to_string();
        if include_hidden || !file_name.starts_with('.') {
            file_paths.push(file_name);
        }
    }
    Ok(file_paths)
}

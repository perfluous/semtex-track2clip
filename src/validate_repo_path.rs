use std::path::Path;
use git2::Repository;

pub fn validate_repo_path(repo_path: &str) -> Result<(), &'static str> {
    let path = Path::new(repo_path);

    if !path.exists() || !path.is_dir() {
        return Err("Error: The provided path is not a valid directory.");
    }

    if !Repository::open(path).is_ok() {
        return Err("Error: The directory is not a valid Git repository.");
    }

    Ok(())
}

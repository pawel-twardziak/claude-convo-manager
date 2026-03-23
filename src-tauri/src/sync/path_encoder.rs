use std::path::{Path, PathBuf};

pub fn get_claude_dir() -> PathBuf {
    if let Ok(dir) = std::env::var("CLAUDE_DIR") {
        return PathBuf::from(dir);
    }
    dirs::home_dir()
        .expect("Could not determine home directory")
        .join(".claude")
}

pub fn get_projects_dir() -> PathBuf {
    get_claude_dir().join("projects")
}

pub fn extract_display_name(project_path: &str) -> String {
    Path::new(project_path)
        .file_name()
        .map(|s| s.to_string_lossy().to_string())
        .unwrap_or_else(|| project_path.to_string())
}

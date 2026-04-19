use std::fs;
use std::path::PathBuf;

use crate::task::Task;

fn task_file_path() -> PathBuf {
    // Default to ~/.cache/.rustdo.json
    let cache = dirs::cache_dir().expect("Could not determine cache directory");
    cache.join(".rustdo.json")
}

pub fn load_tasks() -> Result<Vec<Task>, Box<dyn std::error::Error>> {
    let path = task_file_path();
    if !path.exists() {
        return Ok(Vec::new());
    }
    let content = fs::read_to_string(&path)?;
    let tasks: Vec<Task> = serde_json::from_str(&content)?;
    Ok(tasks)
}

pub fn save_tasks(tasks: &[Task]) -> Result<(), Box<dyn std::error::Error>> {
    let path = task_file_path();
    let content = serde_json::to_string_pretty(tasks)?;
    fs::write(path, content)?;
    Ok(())
}

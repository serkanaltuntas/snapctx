use anyhow::Result;
use std::path::{Path, PathBuf};
use walkdir::{DirEntry, WalkDir};
use std::collections::HashSet;

#[derive(Debug, Clone)]
pub struct Project {
    pub path: PathBuf,
    pub name: String,
    ignored_patterns: HashSet<String>,
}

impl Project {
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self> {
        let path = path.as_ref().canonicalize()?;
        let name = path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("unknown")
            .to_string();

        let mut ignored_patterns = HashSet::new();
        ignored_patterns.insert(".git".to_string());
        ignored_patterns.insert("target".to_string());
        ignored_patterns.insert("node_modules".to_string());
        ignored_patterns.insert("__pycache__".to_string());

        Ok(Self {
            path,
            name,
            ignored_patterns,
        })
    }

    pub fn list_files(&self) -> Result<Vec<PathBuf>> {
        let mut files = Vec::new();
        let walker = WalkDir::new(&self.path)
            .min_depth(0)
            .follow_links(false)
            .into_iter();

        for entry in walker {
            let entry = entry?;
            let is_ignored = self.should_ignore(&entry);

            #[cfg(test)]
            println!("Checking {}: ignored={}", entry.path().display(), is_ignored);

            if !is_ignored && entry.file_type().is_file() {
                files.push(entry.path().to_owned());
            }
        }

        Ok(files)
    }

    pub fn should_ignore(&self, entry: &DirEntry) -> bool {
        if let Some(file_name) = entry.file_name().to_str() {
            // Special handling for dot files and .pyc files
            if file_name.starts_with('.') || file_name.ends_with(".pyc") {
                return true;
            }

            // Check if the file or any of its parent directories should be ignored
            let path = entry.path();
            path.components().any(|component| {
                if let Some(name) = component.as_os_str().to_str() {
                    self.ignored_patterns.contains(name)
                } else {
                    false
                }
            })
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    // Helper function to list files in a directory, used for debugging
    fn list_all_files(dir: &Path) -> Result<Vec<PathBuf>> {
        let mut files = Vec::new();
        for entry in WalkDir::new(dir) {
            let entry = entry?;
            if entry.file_type().is_file() {
                files.push(entry.path().to_owned());
            }
        }
        Ok(files)
    }

    #[test]
    fn test_project_creation() -> Result<()> {
        let temp_dir = TempDir::new()?;
        let project = Project::new(temp_dir.path())?;

        assert_eq!(project.name, temp_dir.path().file_name().unwrap().to_string_lossy());
        Ok(())
    }

    #[test]
    fn test_should_ignore() -> Result<()> {
        let temp_dir = TempDir::new()?;
        let project = Project::new(temp_dir.path())?;

        // Create some test entries
        fs::create_dir_all(temp_dir.path().join("src"))?;
        fs::create_dir_all(temp_dir.path().join(".git"))?;
        fs::create_dir_all(temp_dir.path().join("node_modules"))?;
        fs::create_dir_all(temp_dir.path().join("target"))?;
        fs::create_dir_all(temp_dir.path().join("__pycache__"))?;
        fs::write(temp_dir.path().join("test.pyc"), "")?;
        fs::write(temp_dir.path().join("src/main.rs"), "fn main() {}")?;

        // Get filtered files
        let files = project.list_files()?;
        println!("\nProject path: {}", project.path.display());
        println!("All files in directory:");
        for file in list_all_files(temp_dir.path())? {
            println!("  {}", file.display());
        }
        println!("\nFiles after filtering:");
        for file in &files {
            println!("  {}", file.display());
        }

        // Split found files into just filenames
        let file_names: Vec<String> = files
            .iter()
            .map(|p| p.file_name().unwrap().to_string_lossy().to_string())
            .collect();

        // Test that main.rs is found
        assert!(
            file_names.contains(&"main.rs".to_string()),
            "main.rs should not be ignored. Found files: {:?}", file_names
        );

        // Test that ignored files are not found
        assert!(
            !file_names.contains(&"test.pyc".to_string()),
            "test.pyc should be ignored. Found files: {:?}", file_names
        );

        Ok(())
    }

    #[test]
    fn test_list_files() -> Result<()> {
        let temp_dir = TempDir::new()?;
        let project = Project::new(temp_dir.path())?;

        // Create test files
        fs::create_dir_all(temp_dir.path().join("src"))?;
        fs::write(temp_dir.path().join("Cargo.toml"), "[package]\nname = \"test\"\n")?;
        fs::write(temp_dir.path().join("src/main.rs"), "fn main() {}")?;
        fs::write(temp_dir.path().join("src/lib.rs"), "pub fn test() {}")?;

        println!("Project path: {}", project.path.display());
        println!("All files in directory:");
        for file in list_all_files(temp_dir.path())? {
            println!("  {}", file.display());
        }

        let files = project.list_files()?;
        println!("\nFiles found by list_files:");
        for file in &files {
            println!("  {}", file.display());
        }

        assert_eq!(files.len(), 3, "Expected 3 files, found {}. Files: {:?}",
                   files.len(),
                   files.iter().map(|p| p.display().to_string()).collect::<Vec<_>>()
        );

        let file_names: Vec<String> = files
            .iter()
            .map(|p| p.file_name().unwrap().to_string_lossy().to_string())
            .collect();

        assert!(file_names.contains(&"Cargo.toml".to_string()),
                "Cargo.toml not found in files: {:?}", file_names);
        assert!(file_names.contains(&"main.rs".to_string()),
                "main.rs not found in files: {:?}", file_names);
        assert!(file_names.contains(&"lib.rs".to_string()),
                "lib.rs not found in files: {:?}", file_names);

        Ok(())
    }
}
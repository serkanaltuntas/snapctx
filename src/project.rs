use anyhow::Result;
use ignore::WalkBuilder;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
pub struct Project {
    pub path: PathBuf,
    pub name: String,
}

impl Project {
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self> {
        let path = path.as_ref().canonicalize()?;
        let name = path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("unknown")
            .to_string();

        Ok(Self { path, name })
    }

    pub fn list_files(&self) -> Result<Vec<PathBuf>> {
        let mut files = Vec::new();

        // Configure the walker to respect .gitignore and follow symbolic links
        let walker = WalkBuilder::new(&self.path)
            .hidden(true)         // Respect hidden files
            .git_global(true)     // Use global gitignore
            .git_ignore(true)     // Use .gitignore
            .ignore(true)         // Use .ignore
            .parents(true)        // Look for .gitignore in parent directories
            .build();

        for result in walker {
            match result {
                Ok(entry) => {
                    if entry.file_type().map_or(false, |ft| ft.is_file()) {
                        files.push(entry.path().to_owned());
                    }
                }
                Err(err) => {
                    log::warn!("Error walking directory: {}", err);
                    continue;
                }
            }
        }

        Ok(files)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn test_project_creation() -> Result<()> {
        let temp_dir = TempDir::new()?;
        let project = Project::new(temp_dir.path())?;

        assert_eq!(project.name, temp_dir.path().file_name().unwrap().to_string_lossy());
        Ok(())
    }

    #[test]
    fn test_gitignore_respected() -> Result<()> {
        let temp_dir = TempDir::new()?;
        let project = Project::new(temp_dir.path())?;

        // Create a .gitignore file
        fs::write(
            temp_dir.path().join(".gitignore"),
            "*.log\nnode_modules/\nbuild/\n",
        )?;

        // Create some test files and directories
        fs::create_dir_all(temp_dir.path().join("src"))?;
        fs::create_dir_all(temp_dir.path().join("node_modules"))?;
        fs::create_dir_all(temp_dir.path().join("build"))?;

        fs::write(temp_dir.path().join("src/main.rs"), "fn main() {}")?;
        fs::write(temp_dir.path().join("test.log"), "test log")?;
        fs::write(temp_dir.path().join("Cargo.toml"), "[package]\nname = \"test\"")?;
        fs::write(temp_dir.path().join("build/output.txt"), "build output")?;
        fs::write(temp_dir.path().join("node_modules/package.json"), "{}")?;

        let files = project.list_files()?;
        let file_names: Vec<String> = files
            .iter()
            .map(|p| p.strip_prefix(&project.path).unwrap().to_string_lossy().to_string())
            .collect();

        // These files should be included
        assert!(file_names.contains(&"src/main.rs".to_string()));
        assert!(file_names.contains(&"Cargo.toml".to_string()));

        // These files should be ignored
        assert!(!file_names.contains(&"test.log".to_string()));
        assert!(!file_names.contains(&"build/output.txt".to_string()));
        assert!(!file_names.contains(&"node_modules/package.json".to_string()));

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

        let files = project.list_files()?;
        let file_names: Vec<String> = files
            .iter()
            .map(|p| p.file_name().unwrap().to_string_lossy().to_string())
            .collect();

        assert!(file_names.contains(&"Cargo.toml".to_string()));
        assert!(file_names.contains(&"main.rs".to_string()));
        assert!(file_names.contains(&"lib.rs".to_string()));

        Ok(())
    }
}
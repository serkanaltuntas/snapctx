use anyhow::Result;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

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
        for entry in WalkDir::new(&self.path)
            .follow_links(true)
            .into_iter()
            .filter_entry(|e| !self.should_ignore(e))
        {
            let entry = entry?;
            if entry.file_type().is_file() {
                files.push(entry.path().to_owned());
            }
        }
        Ok(files)
    }

    fn should_ignore(&self, entry: &walkdir::DirEntry) -> bool {
        entry
            .file_name()
            .to_str()
            .map(|s| {
                s.starts_with('.') ||
                    s == "target" ||
                    s == "node_modules" ||
                    s == "__pycache__" ||
                    s.ends_with(".pyc")
            })
            .unwrap_or(false)
    }
}
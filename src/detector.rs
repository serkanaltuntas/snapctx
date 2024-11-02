use anyhow::Result;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum DetectorError {
    #[error("Unable to determine project type")]
    UnknownProjectType,
}

#[derive(Debug, Clone)]
pub enum ProjectType {
    Rust,
    JavaScript,
    Python,
    Unknown,
}

pub struct ProjectDetector {
    markers: Vec<(ProjectType, &'static str)>,
}

impl ProjectDetector {
    pub fn new() -> Self {
        let markers = vec![
            (ProjectType::Rust, "Cargo.toml"),
            (ProjectType::JavaScript, "package.json"),
            (ProjectType::Python, "requirements.txt"),
            (ProjectType::Python, "setup.py"),
        ];
        Self { markers }
    }

    pub fn detect(&self, project: &crate::project::Project) -> Result<ProjectType> {
        for (project_type, marker) in &self.markers {
            if project.path.join(marker).exists() {
                return Ok(project_type.clone());
            }
        }
        Ok(ProjectType::Unknown)
    }
}
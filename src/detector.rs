use anyhow::Result;
use thiserror::Error;
use crate::project::Project;

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

    pub fn detect(&self, project: &Project) -> Result<ProjectType> {
        for (project_type, marker) in &self.markers {
            if project.path.join(marker).exists() {
                return Ok(project_type.clone());
            }
        }
        Ok(ProjectType::Unknown)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;
    use anyhow::Result;

    #[test]
    fn test_detector_new() {
        let detector = ProjectDetector::new();
        assert!(!detector.markers.is_empty());
    }

    #[test]
    fn test_detect_unknown_project() -> Result<()> {
        let temp_dir = TempDir::new()?;
        let project = Project::new(temp_dir.path())?;
        let detector = ProjectDetector::new();

        assert!(matches!(detector.detect(&project)?, ProjectType::Unknown));
        Ok(())
    }
}
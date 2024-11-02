pub mod cli;
pub mod detector;
pub mod output;
pub mod project;

use anyhow::Result;
use cli::Cli;
use crate::detector::ProjectDetector;
use crate::output::OutputGenerator;
use crate::project::Project;

pub fn run(cli: Cli) -> Result<()> {
    let project = Project::new(&cli.project_path)?;
    let detector = ProjectDetector::new();
    let project_type = detector.detect(&project)?;

    let generator = OutputGenerator::new(cli.batch_mode);
    generator.generate(project, project_type)?;

    Ok(())
}
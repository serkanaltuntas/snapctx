use anyhow::Result;
use chrono::Local;
use std::{
    fs,
    io::{self, Write},
    path::PathBuf,
};

use crate::{detector::ProjectType, project::Project};

pub struct OutputGenerator {
    batch_mode: bool,
}

impl OutputGenerator {
    pub fn new(batch_mode: bool) -> Self {
        Self { batch_mode }
    }

    pub fn generate(
        &self,
        project: Project,
        project_type: ProjectType,
    ) -> Result<()> {
        let output = self.create_summary(&project, &project_type)?;
        let output_path = self.get_output_path(&project);

        fs::write(&output_path, output)?;
        println!("Summary written to: {}", output_path.display());

        if !self.batch_mode {
            self.handle_interactive_mode(&output_path)?;
        }

        Ok(())
    }

    fn create_summary(
        &self,
        project: &Project,
        project_type: &ProjectType,
    ) -> Result<String> {
        let mut summary = String::new();

        // Add header
        summary.push_str(&format!(
            "# Project Summary: {}\nGenerated: {}\nType: {:?}\n\n",
            project.name,
            Local::now().format("%Y-%m-%d %H:%M:%S"),
            project_type
        ));

        // Add file tree
        summary.push_str("## Project Structure\n```\n");
        for file in project.list_files()? {
            if let Ok(relative) = file.strip_prefix(&project.path) {
                summary.push_str(&format!("{}\n", relative.display()));
            }
        }
        summary.push_str("```\n\n");

        // Add file contents
        summary.push_str("## File Contents\n");
        for file in project.list_files()? {
            if let Ok(relative) = file.strip_prefix(&project.path) {
                if let Ok(content) = fs::read_to_string(&file) {
                    summary.push_str(&format!(
                        "\n### {}\n```\n{}\n```\n",
                        relative.display(),
                        content
                    ));
                }
            }
        }

        Ok(summary)
    }

    fn get_output_path(&self, project: &Project) -> PathBuf {
        let timestamp = Local::now().format("%Y%m%d_%H%M%S");
        project
            .path
            .join(format!("{}_snapshot_{}.md", project.name, timestamp))
    }

    fn handle_interactive_mode(&self, output_path: &PathBuf) -> Result<()> {
        println!("\nEnter your question/prompt for the LLM (or press Enter to skip):");
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        if !input.trim().is_empty() {
            let mut file = fs::OpenOptions::new()
                .append(true)
                .open(output_path)?;
            writeln!(file, "\n## LLM Prompt\n{}", input.trim())?;
        }

        Ok(())
    }
}
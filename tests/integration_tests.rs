use std::fs;
use std::path::PathBuf;
use clap::Parser;
use tempfile::TempDir;
use snapctx::{
    cli::Cli,
    detector::{ProjectDetector, ProjectType},
    project::Project,
    output::OutputGenerator,
};

// Helper function to create a temporary project structure
fn setup_test_project(project_type: &str) -> anyhow::Result<(TempDir, Project)> {
    let temp_dir = TempDir::new()?;
    let project_path = temp_dir.path();

    // Create src directory first
    fs::create_dir(project_path.join("src"))?;

    // Create project marker file based on type
    match project_type {
        "rust" => {
            fs::write(
                project_path.join("Cargo.toml"),
                "[package]\nname = \"test\"\nversion = \"0.1.0\"\n",
            )?;
            fs::write(project_path.join("src/main.rs"), "fn main() {}\n")?;
            fs::write(project_path.join("src/lib.rs"), "pub fn test() {}\n")?;
        }
        "javascript" => {
            fs::write(project_path.join("package.json"), "{}")?;
        }
        "python" => {
            fs::write(project_path.join("requirements.txt"), "")?;
        }
        _ => {}
    }

    let project = Project::new(project_path)?;
    Ok((temp_dir, project))
}

#[test]
fn test_project_detection() -> anyhow::Result<()> {
    let detector = ProjectDetector::new();

    // Test Rust project detection
    let (temp_dir, rust_project) = setup_test_project("rust")?;
    assert!(matches!(detector.detect(&rust_project)?, ProjectType::Rust));
    drop(temp_dir);

    // Test JavaScript project detection
    let (temp_dir, js_project) = setup_test_project("javascript")?;
    assert!(matches!(detector.detect(&js_project)?, ProjectType::JavaScript));
    drop(temp_dir);

    // Test Python project detection
    let (temp_dir, py_project) = setup_test_project("python")?;
    assert!(matches!(detector.detect(&py_project)?, ProjectType::Python));
    drop(temp_dir);

    // Test unknown project detection
    let (temp_dir, unknown_project) = setup_test_project("unknown")?;
    assert!(matches!(detector.detect(&unknown_project)?, ProjectType::Unknown));
    drop(temp_dir);

    Ok(())
}

#[test]
fn test_project_file_listing() -> anyhow::Result<()> {
    let (temp_dir, project) = setup_test_project("rust")?;

    // Debug print current directory structure
    println!("Project path: {}", project.path.display());
    for entry in fs::read_dir(&project.path)? {
        let entry = entry?;
        println!("Found: {}", entry.path().display());
        if entry.path().is_dir() {
            for subentry in fs::read_dir(entry.path())? {
                let subentry = subentry?;
                println!("  - {}", subentry.path().display());
            }
        }
    }

    let files = project.list_files()?;
    println!("Listed files:");
    for file in &files {
        println!("- {}", file.display());
    }

    // Check that we have the expected number of files
    assert_eq!(files.len(), 3, "Expected 3 files (Cargo.toml, main.rs, lib.rs), found {}", files.len());

    // Check that we have the expected files
    let file_names: Vec<String> = files
        .iter()
        .map(|p| p.file_name().unwrap().to_string_lossy().to_string())
        .collect();

    assert!(
        file_names.contains(&"Cargo.toml".to_string()),
        "Cargo.toml not found in files: {:?}", file_names
    );
    assert!(
        file_names.contains(&"main.rs".to_string()),
        "main.rs not found in files: {:?}", file_names
    );
    assert!(
        file_names.contains(&"lib.rs".to_string()),
        "lib.rs not found in files: {:?}", file_names
    );

    drop(temp_dir);
    Ok(())
}

#[test]
fn test_output_generation() -> anyhow::Result<()> {
    let (temp_dir, project) = setup_test_project("rust")?;
    let generator = OutputGenerator::new(true); // Use batch mode for testing

    generator.generate(project.clone(), ProjectType::Rust)?;

    // Find the generated file
    let output_files: Vec<PathBuf> = fs::read_dir(temp_dir.path())?
        .filter_map(|entry| entry.ok())
        .map(|entry| entry.path())
        .filter(|path| path.extension().map_or(false, |ext| ext == "md"))
        .collect();

    assert_eq!(output_files.len(), 1);

    // Check the content of the generated file
    let content = fs::read_to_string(&output_files[0])?;
    assert!(content.contains("Project Summary"));
    assert!(content.contains("Type: Rust"));
    assert!(content.contains("Project Structure"));
    assert!(content.contains("File Contents"));

    drop(temp_dir);
    Ok(())
}

#[test]
fn test_cli_parsing() {
    let args = vec!["scx", "/test/path", "--batch-mode"];
    let cli = Cli::parse_from(args);

    assert_eq!(cli.project_path, PathBuf::from("/test/path"));
    assert!(cli.batch_mode);
}
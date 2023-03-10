use std::fs::File;
use std::io::Write;
use std::path::Path;

use clap::Parser;
use eyre::Result;
use libpsmt::{ExecutableCommand, ProjectConfig};

/// Initialize a new psmt project
/// in the current directory
#[derive(Parser, Debug)]
#[command()]
pub struct InitCommand {
    #[arg(value_name = "project_dir", default_value_t = String::from("."), required = false)]
    project_directory: String,
}

impl ExecutableCommand for InitCommand {
    fn exec(&self) -> Result<()> {
        // todo!("Not implemented")
        // TODO:
        // Check if psmt is already initialized
        // Show emoji message with an error
        // Ask using inquire crate for the basic info
        // => Design configuration structure
        // TODO: Handle config errors gracefully
        let current_config = ProjectConfig::read()?;
        println!("{:?}", current_config);
        let config = ProjectConfig::default().write()?;
        let config_path = Path::new(&self.project_directory).join("psmt.toml");
        let mut config_file = File::create(config_path)?;
        config_file.write_all(config.as_bytes())?;
        Ok(())
    }
}

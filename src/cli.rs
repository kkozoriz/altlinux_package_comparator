use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(bin_name = "pkg-cmp")]
#[command(version = "0.1.0")]
#[command(about = "Command line arguments for alt linux package comparator", long_about = None)]
pub struct Args {
    #[command(subcommand)]
    command: CliCommands,

    /// Path to the output file
    #[arg(short = 'o', long = "output-file")]
    output_file: Option<PathBuf>,

    /// Architecture to filter the packages (optional)
    #[arg(short = 'a', long = "arch")]
    arch: Option<String>,

    /// Name of first comparable branch
    #[arg(default_value = "sisyphus")]
    first_branch: String,

    /// Name of second comparable branch
    #[arg(default_value = "p10")]
    second_branch: String,
}

#[derive(Subcommand)]
pub enum CliCommands {
    /// Find packages that are only in the first branch
    FirstBranchOnly,
    /// Find packages that are only in the second branch
    SecondBranchOnly,
    /// Find packages where the version-release is newer in the first branch than in the second branch
    PackagesNewer,
}

impl Args {
    pub fn arch(&self) -> &Option<String> {
        &self.arch
    }
    pub fn first_branch(&self) -> &str {
        self.first_branch.as_str()
    }
    pub fn second_branch(&self) -> &str {
        self.second_branch.as_str()
    }
    pub fn command(&self) -> &CliCommands {
        &self.command
    }
    pub fn output_file(&self) -> &Option<PathBuf> {
        &self.output_file
    }
}

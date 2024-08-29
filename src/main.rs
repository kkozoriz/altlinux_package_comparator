use altlinux_package_comparator::cli::{Args, CliCommands};
use altlinux_package_comparator::process_branch_packages;
use clap::Parser;
use dotenvy::dotenv;
use log::{error, info};
use std::path::PathBuf;
use std::{env, process};

#[tokio::main]
async fn main() {
    dotenv().ok();
    env_logger::init();

    let args: Args = Args::parse();

    // Retrieve environment variables and command-line arguments
    let url: &str = &env::var("URL").expect("URL must be set");
    let first_branch: &str = args.first_branch();
    let second_branch: &str = args.second_branch();
    let arch: &str = &args.arch().clone().unwrap_or_default();
    let commands: &CliCommands = args.command();
    let output_file: &Option<PathBuf> = args.output_file();

    match process_branch_packages(
        first_branch,
        second_branch,
        arch,
        url,
        commands,
        output_file,
    )
    .await
    {
        Ok(_) => info!("Application terminated successfully"),
        Err(err) => {
            error!("Application error: {err}");
            process::exit(1);
        }
    }
}

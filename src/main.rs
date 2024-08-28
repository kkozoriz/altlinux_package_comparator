use crate::cli::Args;
use altlinux_package_comparator::process_branch_packages;
use clap::Parser;
use dotenvy::dotenv;
use log::error;
use std::{env, process};

mod cli;

// TODO: Add positive result processing
#[tokio::main]
async fn main() {
    dotenv().ok();
    env_logger::init();

    let args: Args = Args::parse();

    let url: &str = &env::var("URL").expect("URL must be set");
    let first_branch: &str = args.first_branch();
    let second_branch: &str = args.second_branch();
    let arch: &str = &args.arch().clone().unwrap_or_default();

    match process_branch_packages(first_branch, second_branch, arch, url).await {
        Ok(_) => {}
        Err(err) => {
            error!("Application error: {err}");
            process::exit(1);
        }
    }
}

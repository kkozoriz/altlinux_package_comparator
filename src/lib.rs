mod api;
pub mod cli;
mod package;

use crate::api::fetch_packages;
use crate::cli::CliCommands;
use crate::package::Package;

use log::info;
use rpm::rpm_evr_compare;
use std::cmp::Ordering;
use std::io::{self, Write};
use std::{collections::HashSet, fs::File, path::PathBuf};

type AppError = Box<dyn std::error::Error>;

pub async fn process_branch_packages(
    first_branch: &str,
    second_branch: &str,
    arch: &str,
    url: &str,
    command: &CliCommands,
    path: &Option<PathBuf>,
) -> Result<(), AppError> {
    // fetch the package lists for both branches
    let (first_packages_set, second_packages_set): (HashSet<Package>, HashSet<Package>) = tokio::try_join!(
        fetch_packages(first_branch, arch, url),
        fetch_packages(second_branch, arch, url)
    )?;

    info!("Start processing...");
    let result = match command {
        CliCommands::FirstBranchOnly => get_difference(&first_packages_set, &second_packages_set),
        CliCommands::SecondBranchOnly => get_difference(&second_packages_set, &first_packages_set),
        CliCommands::PackagesNewer => {
            get_newer_versions_set(&first_packages_set, &second_packages_set)
        }
    };

    show_output_result(path, result).map_err(|err| Box::new(err) as AppError)
}

fn show_output_result(path: &Option<PathBuf>, result: HashSet<&Package>) -> io::Result<()> {
    let result_vec: Vec<&Package> = result.into_iter().collect();

    // Serialize the vector into a pretty JSON string
    let json_output: String =
        serde_json::to_string_pretty(&result_vec).expect("Failed to serialize JSON");

    match path {
        // If a file path is provided, write the JSON output to the file
        Some(file_path) => {
            let mut file: File = File::create(file_path)?;
            writeln!(file, "{}", json_output)?;
        }
        // Otherwise, print the JSON output to the terminal
        None => println!("{}", json_output),
    }
    Ok(())
}

fn get_difference<'a>(
    set_1: &'a HashSet<Package>,
    set_2: &'a HashSet<Package>,
) -> HashSet<&'a Package> {
    set_1.difference(set_2).collect()
}

fn get_newer_versions_set<'a>(
    first_packages: &'a HashSet<Package>,
    second_packages: &'a HashSet<Package>,
) -> HashSet<&'a Package> {
    first_packages
        .iter()
        .filter_map(|pkg1| {
            second_packages
                .get(pkg1)
                .filter(|pkg2| {
                    let vs_1 = &format!("{}-{}", pkg1.version, pkg1.release);
                    let vs_2 = &format!("{}-{}", pkg2.version, pkg2.release);

                    rpm_evr_compare(vs_1, vs_2) == Ordering::Greater
                })
                .map(|_| pkg1)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    fn create_package(name: &str, version: &str, release: &str, arch: &str) -> Package {
        Package {
            name: name.to_string(),
            version: version.to_string(),
            release: release.to_string(),
            arch: arch.to_string(),
        }
    }

    #[test]
    fn test_get_difference() {
        let pkg1 = create_package("pkg1", "1.0", "1", "x86_64");
        let pkg2 = create_package("pkg2", "1.0", "1", "x86_64");
        let pkg3 = create_package("pkg3", "1.0", "1", "x86_64");

        let mut set_1 = HashSet::new();
        set_1.insert(pkg1.clone());
        set_1.insert(pkg2.clone());

        let mut set_2 = HashSet::new();
        set_2.insert(pkg2.clone());
        set_2.insert(pkg3.clone());

        let difference = get_difference(&set_1, &set_2);

        assert_eq!(difference.len(), 1);
        assert!(difference.contains(&pkg1));
        assert!(!difference.contains(&pkg2));
        assert!(!difference.contains(&pkg3));
    }

    #[test]
    fn test_get_newer_versions_set() {
        let pkg1_v1 = create_package("pkg1", "1.0", "1", "x86_64");
        let pkg1_v2 = create_package("pkg1", "2.0", "1", "x86_64"); // newer version
        let pkg2_v1 = create_package("pkg2", "1.0", "1", "x86_64");
        let pkg2_v2 = create_package("pkg2", "1.0", "2", "x86_64"); // same version, different release

        let mut first_set = HashSet::new();
        first_set.insert(pkg1_v2.clone());
        first_set.insert(pkg2_v1.clone());

        let mut second_set = HashSet::new();
        second_set.insert(pkg1_v1.clone());
        second_set.insert(pkg2_v2.clone());

        let newer_versions = get_newer_versions_set(&first_set, &second_set);

        assert_eq!(newer_versions.len(), 1);
        assert!(newer_versions.contains(&pkg1_v2));
        assert!(!newer_versions.contains(&pkg2_v1));
    }
}

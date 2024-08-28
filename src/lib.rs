use log::info;
use reqwest::{Error, Response};
use serde::{Deserialize, Serialize};
use std::collections::{hash_set::Difference, HashSet};
use std::hash::{Hash, Hasher, RandomState};

#[derive(Debug, Deserialize, Serialize)]
pub struct Package {
    name: String,
    version: String,
    release: String,
    arch: String,
}

// TODO: Put it in a separate module
impl PartialEq for Package {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl Eq for Package {}

impl Hash for Package {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}

#[derive(Debug, Deserialize)]
struct ApiResponse {
    packages: HashSet<Package>,
}

// TODO: Put it in a separate module
async fn fetch_packages(branch: &str, arch: &str, url: &str) -> Result<HashSet<Package>, Error> {
    let response: Response = match arch.is_empty() {
        true => reqwest::get(format!("{}/{}", url, branch)).await?,
        false => reqwest::get(format!("{}/{}?arch={}", url, branch, arch)).await?,
    };
    let api_response: ApiResponse = response.json().await?;

    Ok(api_response.packages)
}

// TODO: Add the ability to output to console or file and await on operations
pub async fn process_branch_packages(
    first_branch: &str,
    second_branch: &str,
    arch: &str,
    url: &str,
) -> Result<(), Error> {
    let first_packages_set: HashSet<Package> = fetch_packages(first_branch, arch, url).await?;
    let second_packages_set: HashSet<Package> = fetch_packages(second_branch, arch, url).await?;

    info!("Start processing...");
    let _diff_1: Difference<Package, RandomState> =
        first_packages_set.difference(&second_packages_set);
    let _diff_2: Difference<Package, RandomState> =
        second_packages_set.difference(&first_packages_set);

    let mut version_le_vec: Vec<&Package> = vec![];

    for pkg1 in &first_packages_set {
        if let Some(pkg2) = second_packages_set.get(pkg1) {
            if pkg1.version > pkg2.version {
                version_le_vec.push(pkg1)
            }
        }
    }

    Ok(())
}

use crate::Package;
use log::debug;
use reqwest::{Error, Response};
use serde::Deserialize;
use std::collections::HashSet;

#[derive(Debug, Deserialize)]
struct ApiResponse {
    packages: HashSet<Package>,
}

pub(crate) async fn fetch_packages(
    branch: &str,
    arch: &str,
    url: &str,
) -> Result<HashSet<Package>, Error> {
    let response: Response = match arch.is_empty() {
        true => reqwest::get(format!("{}/{}", url, branch)).await?,
        false => reqwest::get(format!("{}/{}?arch={}", url, branch, arch)).await?,
    };
    let api_response: ApiResponse = response.json().await?;

    debug!("get packages from branch: {}", branch);
    Ok(api_response.packages)
}

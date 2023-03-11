use std::{
    collections::{btree_map::Entry, BTreeMap},
    sync::Arc,
};

use octocrab::Octocrab;
use thiserror::Error;

use crate::{
    cache::Cache,
    github_repo::GitHubRepo,
    vpm::{Packages, VpmRepos},
};

#[derive(Debug, Default)]
pub struct VpmRepoGenerator {
    octocrab: Arc<Octocrab>,
}

impl VpmRepoGenerator {
    pub fn new(octocrab: impl Into<Arc<Octocrab>>) -> Self {
        Self {
            octocrab: octocrab.into(),
        }
    }

    pub async fn generate(
        &self,
        cache: &mut Cache,
        name: impl Into<String>,
        author: impl Into<String>,
        url: impl Into<String>,
        repos: Vec<GitHubRepo>,
    ) -> Result<VpmRepos, GenerateError> {
        let mut packages: Packages = BTreeMap::new();

        for repo in repos {
            let GitHubRepo { owner, repo } = repo;

            let releases = self
                .octocrab
                .repos(&owner, &repo)
                .releases()
                .list()
                .send()
                .await?;

            for release in releases {
                let users = cache.as_mut();
                let repos = users.entry(owner.to_owned()).or_default();
                let release_tags = repos.entry(repo.to_owned()).or_default();

                let package_json = match release_tags.entry(release.tag_name) {
                    Entry::Vacant(entry) => {
                        let package_json_url = match release
                            .assets
                            .into_iter()
                            .find(|a| a.name == "package.json")
                        {
                            Some(a) => a.browser_download_url,
                            None => continue,
                        };

                        let package_json = reqwest::get(package_json_url).await?.json().await?;

                        entry.insert(package_json)
                    }
                    Entry::Occupied(entry) => entry.into_mut(),
                };

                let package = packages.entry(package_json.name().to_owned()).or_default();
                package
                    .versions
                    .entry(package_json.version().to_owned())
                    .or_insert(package_json.to_owned());
            }
        }

        Ok(VpmRepos {
            name: name.into(),
            author: author.into(),
            url: url.into(),
            packages,
        })
    }
}

#[derive(Debug, Error)]
pub enum GenerateError {
    #[error(transparent)]
    Octocrab(#[from] octocrab::Error),

    #[error(transparent)]
    Reqwest(#[from] reqwest::Error),

    #[error(transparent)]
    SerdeJson(#[from] serde_json::Error),

    #[error(transparent)]
    SemVer(#[from] semver::Error),
}

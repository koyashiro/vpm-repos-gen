use std::{collections::BTreeMap, sync::Arc};

use octocrab::Octocrab;
use thiserror::Error;

use crate::{
    github_repo::GitHubRepo,
    package_json::PackageJson,
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
        name: impl Into<String>,
        author: impl Into<String>,
        url: impl Into<String>,
        id: impl Into<String>,
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
                for json_asset in release
                    .assets
                    .into_iter()
                    .filter(|a| a.content_type == mime::APPLICATION_JSON.as_ref())
                {
                    let Ok(package_json) = reqwest::get(json_asset.browser_download_url)
                        .await?
                        .json::<PackageJson>()
                        .await
                    else {
                        eprintln!(
                            "[{owner}/{repo}@{}] `{}` is not `package.json`, skipped",
                            release.tag_name, json_asset.name
                        );
                        continue;
                    };

                    packages
                        .entry(package_json.name().to_owned())
                        .or_default()
                        .versions
                        .entry(package_json.version().to_owned())
                        .or_insert(package_json);
                }
            }
        }

        Ok(VpmRepos {
            name: name.into(),
            author: author.into(),
            url: url.into(),
            id: id.into(),
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

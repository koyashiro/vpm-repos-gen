use std::{collections::BTreeMap, sync::Arc};

use octocrab::Octocrab;

use crate::{
    error::Error,
    package_json::PackageJson,
    vpm::{Package, Packages, Repo, VpmRepo},
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
        repos: Vec<Repo>,
    ) -> Result<VpmRepo, Error> {
        let mut packages: Packages = BTreeMap::new();

        for repo in repos {
            let Repo { owner, repo } = repo;
            let releases = self
                .octocrab
                .repos(&owner, &repo)
                .releases()
                .list()
                .send()
                .await?;

            for release in releases {
                let tag = &release.tag_name;
                let assets = release.assets.iter().filter(|a| a.name == "package.json");

                for asset in assets {
                    let package_json: PackageJson =
                        reqwest::get(asset.browser_download_url.to_owned())
                            .await?
                            .json()
                            .await?;

                    if tag != package_json.version() && tag.get(1..) != Some(package_json.version())
                    {
                        return Err(Error::InvalidPackageJson);
                    }

                    match packages.get_mut(package_json.name()) {
                        Some(package) => {
                            package
                                .versions
                                .insert(package_json.version().to_string(), package_json);
                        }
                        None => {
                            let mut package = Package {
                                versions: BTreeMap::new(),
                            };
                            let name = package_json.name().to_string();
                            let version = package_json.version().to_string();
                            package.versions.insert(version, package_json);
                            packages.insert(name, package);
                        }
                    }
                }
            }
        }

        Ok(VpmRepo {
            name: name.into(),
            author: author.into(),
            url: url.into(),
            packages,
        })
    }
}

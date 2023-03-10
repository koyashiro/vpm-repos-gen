use std::{collections::BTreeMap, sync::Arc};

use octocrab::Octocrab;

use crate::{
    error::Error,
    package_json::PackageJson,
    release_tag::ReleaseTag,
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
                let package_json_url = match release
                    .assets
                    .into_iter()
                    .find(|a| a.name == "package.json")
                {
                    Some(a) => a.browser_download_url,
                    None => continue,
                };

                let package_json: PackageJson =
                    reqwest::get(package_json_url).await?.json().await?;

                let release_tag: ReleaseTag = release.tag_name.parse()?;
                if package_json.version() != release_tag.as_version() {
                    return Err(Error::InvalidPackageJson);
                }

                match packages.get_mut(package_json.name()) {
                    Some(package) => {
                        package
                            .versions
                            .insert(package_json.version().to_owned(), package_json);
                    }
                    None => {
                        let name = package_json.name().to_owned();
                        let package = Package {
                            versions: {
                                let mut map = BTreeMap::new();
                                map.insert(package_json.version().to_owned(), package_json);
                                map
                            },
                        };
                        packages.insert(name, package);
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

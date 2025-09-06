use std::collections::BTreeMap;

use thiserror::Error;

use crate::{
    github::GitHubClient,
    github_repo::GitHubRepo,
    http::HttpClient,
    vpm::{Packages, VpmRepos},
};

#[derive(Debug, Default)]
pub struct VpmRepoGenerator<G: GitHubClient, H: HttpClient> {
    github_client: G,
    http_client: H,
}

impl<G: GitHubClient, H: HttpClient> VpmRepoGenerator<G, H> {
    pub fn new(github_client: G, http_client: H) -> Self {
        Self {
            github_client,
            http_client,
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

            let releases = self.github_client.fetch_releases(&owner, &repo).await?;

            for release in releases {
                for json_asset in release
                    .assets
                    .into_iter()
                    .filter(|a| a.content_type == mime::APPLICATION_JSON.as_ref())
                {
                    let Ok(package_json) = self
                        .http_client
                        .fetch_package_json(json_asset.browser_download_url)
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

#[cfg(test)]
mod tests {
    use map_macro::btree_map;
    use mockall::predicate::*;
    use semver::Version;
    use serde_json::{from_value, json};

    use crate::{
        github::{Asset, MockGitHubClient, Release},
        http::MockHttpClient,
        vpm::Package,
    };

    use super::*;

    #[tokio::test]
    async fn test_generator_generate() {
        let mut github_client = MockGitHubClient::new();

        github_client
            .expect_fetch_releases()
            .with(eq("jhon"), eq("foo"))
            .times(1)
            .returning(|_, _| {
                Box::pin(async {
                    Ok(vec![
                        Release {
                            tag_name: "v1.0.0".to_string(),
                            assets: vec![
                                Asset {
                                    name: "package.json".to_string(),
                                    content_type: mime::APPLICATION_JSON.to_string(),
                                    browser_download_url: "https://release-assets.githubusercontent.com/github-production-release-asset/70ce8619-c479-4d41-bb14-2ca801344a83".parse().unwrap(),
                                },
                            ],
                        },
                        Release {
                            tag_name: "v1.1.0".to_string(),
                            assets: vec![
                                Asset {
                                    name: "package.json".to_string(),
                                    content_type: mime::APPLICATION_JSON.to_string(),
                                    browser_download_url: "https://release-assets.githubusercontent.com/github-production-release-asset/1939f9e4-ba80-442d-8ae4-7e6ec6af277d".parse().unwrap(),
                                },
                            ],
                        },
                        Release {
                            tag_name: "v2.0.0".to_string(),
                            assets: vec![
                                Asset {
                                    name: "package.json".to_string(),
                                    content_type: mime::APPLICATION_JSON.to_string(),
                                    browser_download_url: "https://release-assets.githubusercontent.com/github-production-release-asset/d24b9e26-17e3-4b30-939b-a10a5d7523b0".parse().unwrap(),
                                },
                            ],
                        },
                    ])
                })
            });

        github_client
            .expect_fetch_releases()
            .with(eq("jhon"), eq("bar"))
            .times(1)
            .returning(|_, _| {
                Box::pin(async {
                    Ok(vec![
                        Release {
                            tag_name: "v1.0.0".to_string(),
                            assets: vec![
                                Asset {
                                    name: "package.json".to_string(),
                                    content_type: mime::APPLICATION_JSON.to_string(),
                                    browser_download_url: "https://release-assets.githubusercontent.com/github-production-release-asset/4e189868-3df1-46f7-a977-093007f97083".parse().unwrap(),
                                },
                            ],
                        },
                    ])
                })
            });

        let mut http_client = MockHttpClient::new();

        http_client
            .expect_fetch_package_json()
            .with(eq("https://release-assets.githubusercontent.com/github-production-release-asset/70ce8619-c479-4d41-bb14-2ca801344a83".parse::<reqwest::Url>().unwrap()))
            .times(1)
            .returning(|_| {
                Box::pin(async {
                    Ok(from_value(json!({
                        "name": "com.jhon.foo",
                        "version": "1.0.0",
                        "description": "Test package foo v1.0.0"
                    })).unwrap())
                })
            });

        http_client
            .expect_fetch_package_json()
            .with(eq("https://release-assets.githubusercontent.com/github-production-release-asset/1939f9e4-ba80-442d-8ae4-7e6ec6af277d".parse::<reqwest::Url>().unwrap()))
            .times(1)
            .returning(|_| {
                Box::pin(async {
                    Ok(from_value(json!({
                        "name": "com.jhon.foo",
                        "version": "1.1.0",
                        "description": "Test package foo v1.1.0"
                    })).unwrap())
                })
            });

        http_client
            .expect_fetch_package_json()
            .with(eq("https://release-assets.githubusercontent.com/github-production-release-asset/d24b9e26-17e3-4b30-939b-a10a5d7523b0".parse::<reqwest::Url>().unwrap()))
            .times(1)
            .returning(|_| {
                Box::pin(async {
                    Ok(from_value(json!({
                        "name": "com.jhon.foo",
                        "version": "2.0.0",
                        "description": "Test package foo v2.0.0"
                    })).unwrap())
                })
            });

        http_client
            .expect_fetch_package_json()
            .with(eq("https://release-assets.githubusercontent.com/github-production-release-asset/4e189868-3df1-46f7-a977-093007f97083".parse::<reqwest::Url>().unwrap()))
            .times(1)
            .returning(|_| {
                Box::pin(async {
                    Ok(from_value(json!({
                        "name": "com.jhon.bar",
                        "version": "1.0.0",
                        "description": "Test package bar v1.0.0"
                    })).unwrap())
                })
            });

        let generator = VpmRepoGenerator::new(github_client, http_client);

        let vpm_repos = generator
            .generate(
                "jhon's vpm repo",
                "jhon",
                "https://example.com/",
                "com.example",
                vec![
                    GitHubRepo {
                        owner: "jhon".to_string(),
                        repo: "foo".to_string(),
                    },
                    GitHubRepo {
                        owner: "jhon".to_string(),
                        repo: "bar".to_string(),
                    },
                ],
            )
            .await;

        assert!(vpm_repos.is_ok());

        assert_eq!(
            VpmRepos {
                name: "jhon's vpm repo".to_string(),
                author: "jhon".to_string(),
                url: "https://example.com/".to_string(),
                id: "com.example".to_string(),
                packages: btree_map! {
                    "com.jhon.foo".parse().unwrap() => Package {
                        versions: btree_map! {
                            Version::new(1, 0, 0) => from_value(json!({
                                "name": "com.jhon.foo",
                                "version": "1.0.0",
                                "description": "Test package foo v1.0.0"
                            })).unwrap(),
                            Version::new(1, 1, 0) => from_value(json!({
                                "name": "com.jhon.foo",
                                "version": "1.1.0",
                                "description": "Test package foo v1.1.0"
                            })).unwrap(),
                            Version::new(2, 0, 0) => from_value(json!({
                                "name": "com.jhon.foo",
                                "version": "2.0.0",
                                "description": "Test package foo v2.0.0"
                            })).unwrap(),
                        },
                    },
                    "com.jhon.bar".parse().unwrap() => Package {
                        versions: btree_map! {
                            Version::new(1, 0, 0) => from_value(json!({
                                "name": "com.jhon.bar",
                                "version": "1.0.0",
                                "description": "Test package bar v1.0.0"
                            })).unwrap(),
                        },
                    },
                },
            },
            vpm_repos.unwrap(),
        );
    }
}

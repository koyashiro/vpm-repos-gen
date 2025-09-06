use octocrab::Octocrab;
use reqwest::Url;

#[derive(Debug)]
pub struct Asset {
    pub name: String,
    pub content_type: String,
    pub browser_download_url: Url,
}

#[derive(Debug)]
pub struct Release {
    pub tag_name: String,
    pub assets: Vec<Asset>,
}

#[async_trait::async_trait]
#[cfg_attr(test, mockall::automock)]
pub trait GitHubClient {
    async fn fetch_releases(
        &self,
        owner: &str,
        repo: &str,
    ) -> Result<Vec<Release>, octocrab::Error>;
}

#[derive(Debug)]
pub struct OctocrabClient {
    octocrab: Octocrab,
}

impl OctocrabClient {
    pub fn new(octocrab: Octocrab) -> OctocrabClient {
        OctocrabClient { octocrab }
    }
}

#[async_trait::async_trait]
impl GitHubClient for OctocrabClient {
    async fn fetch_releases(
        &self,
        owner: &str,
        repo: &str,
    ) -> Result<Vec<Release>, octocrab::Error> {
        let releases = self
            .octocrab
            .repos(owner, repo)
            .releases()
            .list()
            .send()
            .await?
            .into_iter()
            .map(|r| Release {
                tag_name: r.tag_name,
                assets: r
                    .assets
                    .into_iter()
                    .map(|a| Asset {
                        name: a.name,
                        content_type: a.content_type,
                        browser_download_url: a.browser_download_url,
                    })
                    .collect(),
            })
            .collect();

        Ok(releases)
    }
}

use reqwest::Url;

use crate::package_json::PackageJson;

#[async_trait::async_trait]
#[cfg_attr(test, mockall::automock)]
pub trait HttpClient {
    async fn fetch_package_json(&self, url: Url) -> Result<PackageJson, reqwest::Error>;
}

#[derive(Debug)]
pub struct ReqwestClient;

#[async_trait::async_trait]
impl HttpClient for ReqwestClient {
    async fn fetch_package_json(&self, url: Url) -> Result<PackageJson, reqwest::Error> {
        let json = reqwest::get(url).await?.json::<PackageJson>().await?;
        Ok(json)
    }
}

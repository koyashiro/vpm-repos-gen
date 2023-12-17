use std::{collections::BTreeMap, fs::File, io, path::PathBuf};

use thiserror::Error;

use crate::package_json::PackageJson;

#[derive(Debug, Default)]
pub struct Cache(BTreeMap<String, BTreeMap<String, BTreeMap<String, PackageJson>>>);

impl Cache {
    pub const CACHE_FILE: &'static str = "cache.json";

    pub fn cache_dir() -> Result<PathBuf, Error> {
        let cache_dir = dirs::cache_dir()
            .ok_or(Error::CacheDir)?
            .join(env!("CARGO_PKG_NAME"));

        Ok(cache_dir)
    }

    pub fn cache_file_path() -> Result<PathBuf, Error> {
        let cache_file = Self::cache_dir()?.join(Self::CACHE_FILE);

        Ok(cache_file)
    }

    pub fn load() -> Result<Self, Error> {
        let cache_file = match File::open(Self::cache_file_path()?) {
            Ok(f) => f,
            Err(_) => return Ok(Default::default()),
        };
        let cache_map = serde_json::from_reader(cache_file)?;

        Ok(Self(cache_map))
    }

    pub fn save(&self) -> Result<(), Error> {
        std::fs::create_dir_all(Self::cache_dir()?)?;

        let cache_file = File::create(Self::cache_file_path()?)?;
        serde_json::to_writer(cache_file, &self.0)?;

        Ok(())
    }

    pub fn get(
        &mut self,
        owner: impl Into<String>,
        repo: impl Into<String>,
        tag_name: impl AsRef<str>,
    ) -> Option<&PackageJson> {
        self.0
            .entry(owner.into())
            .or_default()
            .entry(repo.into())
            .or_default()
            .get(tag_name.as_ref())
    }
}

#[derive(Debug, Error)]
#[error("cache error")]
pub enum Error {
    #[error("io error")]
    IO(#[from] io::Error),
    #[error("cache dir error")]
    CacheDir,
    #[error("serde error")]
    SerdeJson(#[from] serde_json::Error),
}

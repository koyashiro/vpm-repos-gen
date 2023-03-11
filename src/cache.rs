use std::{collections::BTreeMap, fs::File, io, path::PathBuf};

use thiserror::Error;
use xdg::{BaseDirectories, BaseDirectoriesError};

use crate::package_json::PackageJson;

#[derive(Debug, Default)]
pub struct Cache(Users);

impl AsMut<Users> for Cache {
    fn as_mut(&mut self) -> &mut Users {
        &mut self.0
    }
}

pub type User = String;

pub type Repo = String;

pub type ReleaseTag = String;

pub type Users = BTreeMap<User, Repos>;

pub type Repos = BTreeMap<Repo, ReleaseTags>;

pub type ReleaseTags = BTreeMap<ReleaseTag, PackageJson>;

impl Cache {
    pub const CACHE_FILE: &str = "cache.json";

    pub fn cache_file() -> Result<PathBuf, Error> {
        let xdg_dirs = BaseDirectories::with_prefix(env!("CARGO_PKG_NAME"))?;
        let cache_file = xdg_dirs.place_cache_file(Self::CACHE_FILE)?;

        Ok(cache_file)
    }

    pub fn read() -> Result<Self, Error> {
        let cache_file = match File::open(Self::cache_file()?) {
            Ok(f) => f,
            Err(_) => return Ok(Default::default()),
        };
        let cache_map = serde_json::from_reader(cache_file)?;

        Ok(Self(cache_map))
    }

    pub fn save(&self) -> Result<(), Error> {
        let cache_file = File::create(Self::cache_file()?)?;
        serde_json::to_writer(cache_file, &self.0)?;

        Ok(())
    }
}

#[derive(Debug, Error)]
#[error("cache error")]
pub enum Error {
    #[error("io error")]
    IO(#[from] io::Error),
    #[error("xdg base directories error")]
    BaseDirectories(#[from] BaseDirectoriesError),
    #[error("serde error")]
    SerdeJson(#[from] serde_json::Error),
}

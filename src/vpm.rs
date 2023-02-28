use std::collections::BTreeMap;
use std::str::FromStr;

use semver::Version;
use serde::{Deserialize, Serialize};

use crate::{
    error::Error,
    package_json::{Name, PackageJson},
};

#[derive(Debug, Serialize, Deserialize)]
pub struct VpmRepo {
    pub name: String,
    pub author: String,
    pub url: String,
    pub packages: Packages,
}

pub type Packages = BTreeMap<Name, Package>;

#[derive(Debug, Serialize, Deserialize)]
pub struct Package {
    pub versions: Versions,
}

pub type Versions = BTreeMap<Version, PackageJson>;

#[derive(Clone, Debug)]
pub struct Repo {
    pub owner: String,
    pub repo: String,
}

impl FromStr for Repo {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let splitted: Vec<&str> = s.split('/').collect();
        if splitted.len() != 2 {
            return Err(Error::InvalidRepo);
        }

        let owner = splitted[0].to_string();
        let repo = splitted[1].to_string();

        Ok(Repo { owner, repo })
    }
}

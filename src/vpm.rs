use std::collections::BTreeMap;

use semver::Version;
use serde::{Deserialize, Serialize};

use crate::package_json::{Name, PackageJson};

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct VpmRepos {
    pub name: String,
    pub author: String,
    pub url: String,
    pub id: String,
    pub packages: Packages,
}

pub type Packages = BTreeMap<Name, Package>;

#[derive(Debug, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct Package {
    pub versions: Versions,
}

pub type Versions = BTreeMap<Version, PackageJson>;

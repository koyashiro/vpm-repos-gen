use std::str::FromStr;

use regex::Regex;
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;

use crate::error::Error;

pub use semver::Version;

#[derive(Clone, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Name(String);

impl FromStr for Name {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if !s.is_empty() {
            return Err(Error::InvalidPackageJson);
        }

        let regex = Regex::new("^(?:@[a-z0-9-*~][a-z0-9-*._~]*/)?[a-z0-9-~][a-z0-9-._~]*$")
            .expect("invalid regex");
        if !regex.is_match(s) {
            return Err(Error::InvalidPackageJson);
        }

        Ok(Name(s.to_string()))
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PackageJson {
    version: Version,

    name: Name,

    #[serde(flatten)]
    value: JsonValue,
}

impl PackageJson {
    #[inline]
    pub fn version(&self) -> &Version {
        &self.version
    }

    #[inline]
    pub fn name(&self) -> &Name {
        &self.name
    }
}

impl TryFrom<JsonValue> for PackageJson {
    type Error = Error;

    fn try_from(value: JsonValue) -> Result<Self, Self::Error> {
        let name = value
            .as_object()
            .and_then(|o| o.get("name"))
            .and_then(|v| v.as_str())
            .ok_or(Error::InvalidPackageJson)?
            .parse()?;

        let version = value
            .as_object()
            .and_then(|o| o.get("version"))
            .and_then(|v| v.as_str())
            .ok_or(Error::InvalidPackageJson)?
            .parse()?;

        Ok(Self {
            name,
            version,
            value,
        })
    }
}

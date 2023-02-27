use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;

use crate::error::Error;

#[derive(Debug, Deserialize, Serialize)]
pub struct PackageJson {
    version: String,

    name: String,

    #[serde(flatten)]
    value: JsonValue,
}

impl PackageJson {
    #[inline]
    pub fn version(&self) -> &str {
        &self.version
    }

    #[inline]
    pub fn name(&self) -> &str {
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
            .to_string();

        let version = value
            .as_object()
            .and_then(|o| o.get("version"))
            .and_then(|v| v.as_str())
            .ok_or(Error::InvalidPackageJson)?
            .to_string();

        Ok(Self {
            name,
            version,
            value,
        })
    }
}

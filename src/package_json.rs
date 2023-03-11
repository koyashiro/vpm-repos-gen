mod name;

use semver::Version;
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use thiserror::Error;

pub use name::{Name, ParseError};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
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

#[derive(Debug, Error, Eq, PartialEq)]
#[error("invalid json")]
pub enum TryFromError {
    #[error("must be object")]
    Json,
    #[error("`name` field must be string")]
    NameType,
    #[error("invalid `name` field")]
    NamePattern,
    #[error("`version` field must be string")]
    VersionType,
    #[error("invalid `version` field")]
    VersionPattern,
}

impl TryFrom<JsonValue> for PackageJson {
    type Error = TryFromError;

    fn try_from(value: JsonValue) -> Result<Self, Self::Error> {
        let obj = value.as_object().ok_or(TryFromError::Json)?;

        let name = {
            let name_str = obj
                .get("name")
                .and_then(|n| n.as_str())
                .ok_or(TryFromError::NameType)?;

            name_str.parse().map_err(|_| TryFromError::NamePattern)?
        };

        let version = {
            let version_str = obj
                .get("version")
                .and_then(|n| n.as_str())
                .ok_or(TryFromError::VersionType)?;

            version_str
                .parse()
                .map_err(|_| TryFromError::VersionPattern)?
        };

        Ok(Self {
            name,
            version,
            value,
        })
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    #[test]
    fn package_json_from_str_test() {
        assert_eq!(
            Ok(PackageJson {
                name: "foo".parse().unwrap(),
                version: Version::new(1, 2, 3),
                value: json!({"name": "foo","version": "1.2.3"}),
            }),
            PackageJson::try_from(json!({"name": "foo","version": "1.2.3"})),
        );

        assert_eq!(
            Err(TryFromError::Json),
            PackageJson::try_from(json!("invalid")),
        );

        assert_eq!(
            Err(TryFromError::NameType),
            PackageJson::try_from(json!({"name": 123,"version": "1.2.3"})),
        );

        assert_eq!(
            Err(TryFromError::NamePattern),
            PackageJson::try_from(json!({"name": ".","version": "1.2.3"})),
        );

        assert_eq!(
            Err(TryFromError::VersionType),
            PackageJson::try_from(json!({"name": "foo","version": 123})),
        );

        assert_eq!(
            Err(TryFromError::VersionPattern),
            PackageJson::try_from(json!({"name": "foo","version": "invalid"})),
        );
    }
}

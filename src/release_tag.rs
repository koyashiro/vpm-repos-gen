use std::str::FromStr;

use semver::Version;
use thiserror::Error;

#[derive(Debug, Eq, PartialEq)]
pub struct ReleaseTag(Version);

impl ReleaseTag {
    pub fn as_version(&self) -> &Version {
        self.as_ref()
    }
}

impl FromStr for ReleaseTag {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let Some(b'v') = s.as_bytes().first() else {
            return Err(ParseError);
        };

        let version = s[1..].parse().map_err(|_| ParseError)?;

        Ok(Self(version))
    }
}

impl From<ReleaseTag> for Version {
    fn from(v: ReleaseTag) -> Self {
        v.0
    }
}

impl AsRef<Version> for ReleaseTag {
    fn as_ref(&self) -> &Version {
        &self.0
    }
}

#[derive(Debug, Error, Eq, PartialEq)]
#[error("invalid release tag")]
pub struct ParseError;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_str_test() {
        assert_eq!(
            Ok(ReleaseTag(Version::new(1, 2, 3))),
            ReleaseTag::from_str("v1.2.3")
        );

        assert_eq!(Err(ParseError), ReleaseTag::from_str("1.2.3"));

        assert_eq!(Err(ParseError), ReleaseTag::from_str("invalid"));
    }
}

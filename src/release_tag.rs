use std::str::FromStr;

use semver::Version;

use crate::error::Error;

#[derive(Debug)]
pub struct ReleaseTag(Version);

impl ReleaseTag {
    pub fn as_version(&self) -> &Version {
        self.as_ref()
    }
}

impl FromStr for ReleaseTag {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let Some(b'v') = s.as_bytes().first() else {
            return Err(Error::InvalidReleaseTag);
        };

        let version = s[1..].parse()?;

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

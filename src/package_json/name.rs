use std::str::FromStr;

use regex::Regex;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Clone, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Name(String);

impl FromStr for Name {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            return Err(ParseError);
        }

        let regex = Regex::new("^(?:@[a-z0-9-*~][a-z0-9-*._~]*/)?[a-z0-9-~][a-z0-9-._~]*$")
            .expect("invalid regex");
        if !regex.is_match(s) {
            return Err(ParseError);
        }

        Ok(Name(s.to_string()))
    }
}

#[derive(Debug, Error, Eq, PartialEq)]
#[error("invalid name")]
pub struct ParseError;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_str_test() {
        assert_eq!(Ok(Name("foo".to_string())), Name::from_str("foo"));

        assert_eq!(Err(ParseError), Name::from_str("."));
    }
}

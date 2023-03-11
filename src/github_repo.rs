use std::str::FromStr;

use thiserror::Error;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GitHubRepo {
    pub owner: String,
    pub repo: String,
}

impl FromStr for GitHubRepo {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let splitted: Vec<&str> = s.split('/').collect();
        if splitted.len() != 2 {
            return Err(ParseError);
        }

        let owner = splitted[0].to_string();
        let repo = splitted[1].to_string();

        Ok(GitHubRepo { owner, repo })
    }
}

#[derive(Debug, Error, Eq, PartialEq)]
#[error("invalid GitHub repo")]
pub struct ParseError;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_str_test() {
        assert_eq!(
            Ok(GitHubRepo {
                owner: "octocat".to_string(),
                repo: "hello-world".to_string()
            }),
            GitHubRepo::from_str("octocat/hello-world")
        );

        assert_eq!(Err(ParseError), GitHubRepo::from_str("invalid"));
    }
}

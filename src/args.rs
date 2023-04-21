use std::str::FromStr;

use clap::Parser;

use crate::github_repo::{GitHubRepo, ParseError};

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[arg(long, default_value = "")]
    pub name: String,

    #[arg(long, default_value = "")]
    pub author: String,

    #[arg(long, default_value = "")]
    pub url: String,

    #[arg(long, default_value = "")]
    pub id: String,

    #[arg(long, value_parser = parse_repos)]
    pub repos: Vec<GitHubRepo>,
}

fn parse_repos(arg: &str) -> Result<Vec<GitHubRepo>, ParseError> {
    let args: Vec<&str> = arg.split(',').map(|s| s.trim()).collect();
    if args.len() == 1 && args[0].is_empty() {
        return Ok(Default::default());
    }
    args.into_iter().map(FromStr::from_str).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_repos_test() {
        assert_eq!(parse_repos(""), Ok(vec![]));
        assert_eq!(
            parse_repos("octocat/linguist"),
            Ok(vec![GitHubRepo {
                owner: "octocat".to_string(),
                repo: "linguist".to_string(),
            }])
        );
        assert_eq!(
            parse_repos("octocat/linguist"),
            Ok(vec![GitHubRepo {
                owner: "octocat".to_string(),
                repo: "linguist".to_string(),
            }])
        );
        assert_eq!(
            parse_repos("octocat/linguist,octocat/hello-world"),
            Ok(vec![
                GitHubRepo {
                    owner: "octocat".to_string(),
                    repo: "linguist".to_string(),
                },
                GitHubRepo {
                    owner: "octocat".to_string(),
                    repo: "hello-world".to_string(),
                },
            ])
        );
    }
}

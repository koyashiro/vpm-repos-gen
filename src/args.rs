use std::{
    fs::File,
    io::{self, Write},
};

use clap::Parser;
use octocrab::Octocrab;

use crate::{github_repo::GitHubRepo, vpm::VpmRepos};

pub const STDIN: &str = "-";

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[arg(long, env, default_value = "")]
    pub name: String,

    #[arg(long, env, default_value = "")]
    pub author: String,

    #[arg(long, env, default_value = "")]
    pub url: String,

    #[arg(long, env, default_value = "")]
    pub id: String,

    #[arg(long, env, value_delimiter = ',')]
    pub repos: Vec<GitHubRepo>,

    #[arg(long, short, env, default_value = STDIN)]
    pub output: String,

    #[arg(long, env, overrides_with = "no_pretty")]
    pub pretty: bool,

    #[arg(long, env, overrides_with = "pretty")]
    pub no_pretty: bool,

    #[arg(long, env)]
    pub github_token: Option<String>,
}

impl Args {
    pub fn writer(&self) -> Result<Box<dyn Write>, io::Error> {
        if self.output.as_str() == STDIN {
            Ok(Box::new(io::stdout()))
        } else {
            Ok(Box::new(File::create(&self.output)?))
        }
    }

    pub fn write_json_fn(&self) -> fn(Box<dyn Write>, &VpmRepos) -> serde_json::Result<()> {
        if self.no_pretty {
            serde_json::to_writer
        } else {
            serde_json::to_writer_pretty
        }
    }

    pub fn octocrab(&self) -> Result<Octocrab, octocrab::Error> {
        if let Some(github_token) = &self.github_token {
            Octocrab::builder()
                .personal_token(github_token.clone())
                .build()
        } else {
            Ok(Octocrab::default())
        }
    }
}

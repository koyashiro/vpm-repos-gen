use clap::Parser;

use crate::github_repo::GitHubRepo;

pub const STDIN: &str = "-";

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

    #[arg(long, value_delimiter = ',')]
    pub repos: Vec<GitHubRepo>,

    #[arg(long, short, default_value = STDIN)]
    pub output: String,
}

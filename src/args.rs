use clap::Parser;

use crate::vpm::Repo;

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[arg(long)]
    pub name: String,

    #[arg(long)]
    pub author: String,

    #[arg(long)]
    pub url: String,

    #[arg(long)]
    pub repos: Vec<Repo>,
}

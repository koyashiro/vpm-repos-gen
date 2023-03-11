mod args;
mod error;
mod generator;
mod package_json;
mod release_tag;
mod vpm;

use std::io;

use clap::Parser;

use crate::{args::Args, error::Error, generator::VpmRepoGenerator};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let args = Args::parse();

    let octocrab = octocrab::instance();
    let generator = VpmRepoGenerator::new(octocrab);

    let vpm_repos = generator
        .generate(args.name, args.author, args.url, args.repos)
        .await?;

    serde_json::to_writer_pretty(io::stdout(), &vpm_repos)?;

    Ok(())
}

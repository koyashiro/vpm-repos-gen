mod args;
mod cache;
mod generator;
mod github_repo;
mod package_json;
mod vpm;

use std::io;

use clap::Parser;

use crate::{args::Args, cache::Cache, generator::VpmRepoGenerator};

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let args = Args::parse();

    let octocrab = octocrab::instance();
    let generator = VpmRepoGenerator::new(octocrab);
    let mut cache = Cache::load()?;

    let vpm_repos = generator
        .generate(
            &mut cache,
            args.name,
            args.author,
            args.url,
            args.id,
            args.repos,
        )
        .await?;

    cache.save()?;

    serde_json::to_writer_pretty(io::stdout(), &vpm_repos)?;

    Ok(())
}

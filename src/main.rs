mod args;
mod cache;
mod generator;
mod github_repo;
mod package_json;
mod vpm;

use std::{fs::File, io};

use clap::Parser;

use crate::{
    args::{Args, STDIN},
    cache::Cache,
    generator::VpmRepoGenerator,
};

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

    if args.output.as_str() == STDIN {
        serde_json::to_writer_pretty(io::stdout(), &vpm_repos)?;
    } else {
        let f = File::create(&args.output)?;
        serde_json::to_writer_pretty(f, &vpm_repos)?;
    }

    Ok(())
}

mod args;
mod cache;
mod generator;
mod github_repo;
mod package_json;
mod vpm;

use clap::Parser;

use crate::{args::Args, cache::Cache, generator::VpmRepoGenerator};

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let args = Args::parse();

    let octocrab = octocrab::instance();
    let generator = VpmRepoGenerator::new(octocrab);
    let mut cache = Cache::load()?;

    let writer = args.writer()?;
    let write_json = args.write_json_fn();

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

    write_json(writer, &vpm_repos)?;

    Ok(())
}

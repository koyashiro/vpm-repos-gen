mod args;
mod generator;
mod github_repo;
mod package_json;
mod vpm;

use clap::Parser;

use crate::{args::Args, generator::VpmRepoGenerator};

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let args = Args::parse();

    let octocrab = octocrab::instance();
    let generator = VpmRepoGenerator::new(octocrab);


    let writer = args.writer()?;
    let write_json = args.write_json_fn();

    let vpm_repos = generator
        .generate(args.name, args.author, args.url, args.id, args.repos)
        .await?;

    write_json(writer, &vpm_repos)?;

    Ok(())
}

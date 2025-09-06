mod args;
mod generator;
mod github;
mod github_repo;
mod http;
mod package_json;
mod vpm;

use clap::Parser;

use crate::{args::Args, generator::VpmRepoGenerator, github::OctocrabClient, http::ReqwestClient};

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let args = Args::parse();

    let github_client = OctocrabClient::new(args.octocrab()?);
    let http_cliet = ReqwestClient;
    let generator = VpmRepoGenerator::new(github_client, http_cliet);

    let writer = args.writer()?;
    let write_json_fn = args.write_json_fn();

    let vpm_repos = generator
        .generate(args.name, args.author, args.url, args.id, args.repos)
        .await?;

    write_json_fn(writer, &vpm_repos)?;

    Ok(())
}

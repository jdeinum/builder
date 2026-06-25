use anyhow::{Context, Result};
use cargo_metadata::MetadataCommand;
use git2::{Commit, Oid, Repository, Sort};
use std::collections::HashMap;
use tracing::{error, info};

fn main() {
    tracing_subscriber::fmt::init();
    match run() {
        Ok(_) => {}
        Err(e) => error!("Error running the build tool: {e:?}"),
    };
}

fn run() -> Result<()> {
    let repo: Repository = Repository::init(".").context("open repo")?;

    // get the head of the current branch
    let head = repo.head().context("get head of repo")?;

    // find the list of commits present locally but not origin/master
    let mut revwalk = repo.revwalk().context("create revwalk")?;

    // we want the last commit present on origin master so we can find what changed in our first commit
    let commit_range = format!("origin/master..HEAD");
    revwalk
        .push_range(&commit_range)
        .context("revwalk commit range")?;
    revwalk.set_sorting(Sort::TOPOLOGICAL | Sort::REVERSE)?;
    let x: Result<Vec<Oid>, git2::Error> = revwalk.into_iter().collect();
    let x = x.context("get oids")?;
    let commits: Result<Vec<Commit>, git2::Error> =
        x.iter().map(|c| repo.find_commit(*c)).collect();
    let commits = commits.context("get commits from oids")?;

    info!("Commits: {commits:?}");

    // get the list of crates in this worktree
    let metadata = MetadataCommand::new().exec().unwrap();
    let members = metadata.workspace_members;
    info!("Members: {members:?}");

    // get the list of updated services for each commit
    // to do this, we'll walk each commit, and check which services changed
    let mut changed_services: HashMap<Commit, Vec<String>> = HashMap::new();
    for commit in &commits {
        // find the files changed for this commit
        commit.p
    }

    // suggest a list of version bumps for each service changed

    // allow the user to override the version bump for each service

    // generate the changelog entry

    // commit the changelog and version bumps

    // push to the remote

    // open PR

    Ok(())
}

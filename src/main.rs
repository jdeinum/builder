// Current Goals

use anyhow::{Context, Result};
use git2::{Repository, Sort};
use tracing::error;

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
    // let head = repo.head().context("get head of repo")?;

    // find the list of commits present locally but not origin/master
    let mut revwalk = repo.revwalk().context("create revwalk")?;
    let commit_range = format!("origin/master..HEAD");
    revwalk
        .push_range(&commit_range)
        .context("revwalk commit range")?;

    revwalk.set_sorting(Sort::TOPOLOGICAL | Sort::REVERSE)?;

    for oid in revwalk {
        let oid = oid?;
        let commit = repo.find_commit(oid)?;
        println!("{:?} {:?}", oid, commit.summary().unwrap())
    }

    // get the list of updated servics for each commit

    // suggest a list of version bumps for each service changed

    // allow the user to override the version bump for each service

    // generate the changelog entry

    // commit the changelog and version bumps

    // push to the remote

    // open PR

    Ok(())
}

fn get_head() -> Result<String> {
    todo!()
}

fn get_commits() -> Result<Vec<String>> {
    todo!()
}

fn get_changed_services() -> Result<String> {
    todo!()
}

fn bump_service_version(name: String) -> Result<()> {
    todo!()
}

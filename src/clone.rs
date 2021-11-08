use git2::build::RepoBuilder;
use git2::{FetchOptions, RemoteCallbacks, Repository};
use std::path::Path;

pub fn clone_repo(path: &Path, url: &str) -> Repository {
    let mut builder = RepoBuilder::new();
    let callbacks = RemoteCallbacks::new();
    let mut fetch_options = FetchOptions::new();

    fetch_options.remote_callbacks(callbacks);
    builder.fetch_options(fetch_options);

    let repo = match builder.clone(url, path) {
        Ok(repo) => repo,
        Err(e) => panic!("could not clone repo: {}", e),
    };

    repo
}

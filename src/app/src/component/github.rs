use std::fs::remove_dir_all;
use std::path::{Path, PathBuf};

use async_trait::async_trait;
use git2::Repository;
use hubcaps::{Credentials, Github};
use log::debug;

use huber_common::file::is_empty_dir;
use huber_common::model::package::Package;
use huber_common::model::release::Release;
use huber_common::result::Result;

const HUBER_GITHUB_REPO: &str = "https://github.com/innobead/huber";

#[async_trait]
pub(crate) trait GithubClientTrait {
    async fn get_latest_release(&self, owner: &str, repo: &str, pkg: &Package) -> Result<Release>;
    async fn get_release(
        &self,
        owner: &str,
        repo: &str,
        tag: &str,
        pkg: &Package,
    ) -> Result<Release>;
    async fn get_releases(&self, owner: &str, repo: &str, pkg: &Package) -> Result<Vec<Release>>;
    async fn download_artifacts<P: AsRef<Path> + Send>(
        &self,
        release: &Release,
        dir: P,
    ) -> Result<()>;
    async fn clone<P: AsRef<Path> + Send>(&self, owner: &str, repo: &str, dir: P) -> Result<()>;
}

#[derive(Debug)]
pub(crate) struct GithubClient {
    github: Github,
    git_ssh_key: Option<PathBuf>,
}

impl GithubClient {
    pub(crate) fn new(
        github_credentials: Option<Credentials>,
        git_ssh_key: Option<PathBuf>,
    ) -> Self {
        Self {
            github: Github::new("huber", github_credentials).unwrap(),
            git_ssh_key,
        }
    }

    fn fetch_merge_repo<P: AsRef<Path>>(&self, dir: P) -> Result<()> {
        debug!("Merging huber repo update");

        let repo = Repository::open(dir)?;

        // fetch the origin
        let mut remote = repo.find_remote("origin")?;
        remote.fetch(&["master"], None, None)?;
        let fetch_head = repo.find_reference("FETCH_HEAD")?;
        let commit = repo.reference_to_annotated_commit(&fetch_head)?;

        // merge local, and checkout
        let reference_name = format!("refs/heads/{}", "master");
        let mut reference = repo.find_reference(&reference_name)?;
        let name = reference.name().expect("");
        repo.set_head(name)?;
        reference.set_target(commit.id(), "")?;

        Ok(repo.checkout_head(Some(git2::build::CheckoutBuilder::default().force()))?)
    }
}

#[async_trait]
impl GithubClientTrait for GithubClient {
    async fn get_latest_release(&self, owner: &str, repo: &str, pkg: &Package) -> Result<Release> {
        debug!("Getting the latest release of package {}", &pkg);

        let release = self.github.repo(owner, repo).releases().latest().await?;
        let mut release = Release::from(release);

        release.name = pkg.name.clone();
        release.package.name = pkg.name.clone();
        release.package.source = pkg.source.clone();
        release.package.targets = pkg.targets.clone();

        Ok(release)
    }

    async fn get_release(
        &self,
        owner: &str,
        repo: &str,
        tag: &str,
        pkg: &Package,
    ) -> Result<Release> {
        debug!("Getting the specific release of package {}/{}", &pkg, tag);

        let release = self.github.repo(owner, repo).releases().by_tag(tag).await?;
        let mut release = Release::from(release);

        release.name = pkg.name.clone();
        release.package.name = pkg.name.clone();
        release.package.source = pkg.source.clone();
        release.package.targets = pkg.targets.clone();

        Ok(release)
    }

    async fn get_releases(&self, owner: &str, repo: &str, pkg: &Package) -> Result<Vec<Release>> {
        debug!("Getting all releases of package {}", &pkg);

        let releases = self.github.repo(owner, repo).releases().list().await?;
        let releases = releases
            .into_iter()
            .map(|it| {
                let mut release = Release::from(it);

                release.name = pkg.name.clone();
                release.package.name = pkg.name.clone();
                release.package.source = pkg.source.clone();
                release.package.targets = pkg.targets.clone();
                release.package.release_kind = release.kind.clone();

                release
            })
            .collect();

        Ok(releases)
    }

    async fn download_artifacts<P: AsRef<Path> + Send>(
        &self,
        _release: &Release,
        _dir: P,
    ) -> Result<()> {
        unimplemented!()
    }

    async fn clone<P: AsRef<Path> + Send>(&self, owner: &str, repo: &str, dir: P) -> Result<()> {
        debug!("Cloning huber github repo");

        let url = format!("https://github.com/{}/{}", owner, repo);

        if is_empty_dir(&dir) {
            //Note: if encountering authentication required, probably hit this issue https://github.com/rust-lang/git2-rs/issues/463
            Repository::clone(&url, dir)?;
            return Ok(());
        }

        if let Err(_) = self.fetch_merge_repo(&dir) {
            let _ = remove_dir_all(&dir);
            Repository::clone(&url, dir)?;
            return Ok(());
        }

        Ok(())
    }
}

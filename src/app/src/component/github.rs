use std::env;
use std::fs::remove_dir_all;
use std::path::{Path, PathBuf};

use async_trait::async_trait;
use git2::Repository;
use hubcaps::{Credentials, Github};

use huber_common::file::is_empty_dir;
use huber_common::model::release::Release;
use huber_common::result::Result;

const HUBER_GITHUB_REPO: &str = "https://github.com/innobead/huber";

#[async_trait]
pub(crate) trait GithubClientTrait {
    async fn get_latest_release(&self, owner: &str, repo: &str) -> Result<Release>;
    async fn get_release(&self, owner: &str, repo: &str, tag: &str) -> Result<Release>;
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
    git_ssh_key: Option<PathBuf>
}

impl GithubClient {
    pub(crate) fn new(github_credentials: Option<Credentials>, git_ssh_key: Option<PathBuf>) -> Self {
        let mut credentials = github_credentials;

        if credentials.is_none() {
            let token = env::var("GITHUB_TOKEN").unwrap_or("".to_string());
            credentials = Some(Credentials::Token(token));
        }

        Self {
            github: Github::new("huber", credentials).unwrap(),
            git_ssh_key,
        }
    }

    fn fetch_merge_repo<P: AsRef<Path>>(&self, dir: P) -> Result<()> {
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
    async fn get_latest_release(&self, owner: &str, repo: &str) -> Result<Release> {
        let _release = self.github.repo(owner, repo).releases().latest().await?;

        unimplemented!()
        // FIXME
        /*Ok(Package {
            name: repo.to_string(),
            source: PackageSource::Github {
                owner: owner.to_string(),
                repo: repo.to_string(),
            },
            detail: Some(PackageDetailType::Github {
                release: release.into(),
            }),
            targets: vec![],
        })*/
    }

    async fn get_release(&self, owner: &str, repo: &str, tag: &str) -> Result<Release> {
        let _release = self.github.repo(owner, repo).releases().by_tag(tag).await?;

        unimplemented!()
        //FIXME
        /*Ok(Package {
            name: repo.to_string(),
            source: PackageSource::Github {
                owner: owner.to_string(),
                repo: repo.to_string(),
            },
            detail: Some(PackageDetailType::Github {
                release: release.into(),
            }),
            targets: vec![],
        })*/
    }

    async fn download_artifacts<P: AsRef<Path> + Send>(
        &self,
        _release: &Release,
        _dir: P,
    ) -> Result<()> {
        unimplemented!()
    }

    async fn clone<P: AsRef<Path> + Send>(&self, owner: &str, repo: &str, dir: P) -> Result<()> {
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

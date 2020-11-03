use std::env;
use std::fs::remove_dir_all;
use std::path::Path;

use async_trait::async_trait;
use git2::Repository;
use hubcaps::{Credentials, Github};

use huber_common::file::is_empty_dir;
use huber_common::model::package::{Package, PackageDetailType, PackageSource};
use huber_common::result::Result;

const HUBER_GITHUB_REPO: &str = "https://github.com/innobead/huber";

#[async_trait]
pub(crate) trait GithubClientTrait {
    async fn get_latest_release(&self, owner: &str, repo: &str) -> Result<Package>;
    async fn get_release(&self, owner: &str, repo: &str, tag: &str) -> Result<Package>;
    async fn download_artifacts<P: AsRef<Path> + Send>(
        &self,
        release: &Package,
        dir: P,
    ) -> Result<()>;
    async fn clone<P: AsRef<Path> + Send>(&self, owner: &str, repo: &str, dir: P) -> Result<()>;
    async fn list_managed_releases(&self) -> Result<Vec<Package>>;
}

#[derive(Debug)]
pub(crate) struct GithubClient {
    github: Github,
    //FIXME also add git credentials
}

impl GithubClient {
    pub(crate) fn new(credentials: Option<Credentials>) -> Self {
        let mut credentials = credentials;

        if credentials.is_none() {
            let token = env::var("GITHUB_TOKEN").unwrap_or("".to_string());
            credentials = Some(Credentials::Token(token));
        }

        Self {
            github: Github::new("huber", credentials).unwrap(),
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
    async fn get_latest_release(&self, owner: &str, repo: &str) -> Result<Package> {
        let release = self.github.repo(owner, repo).releases().latest().await?;

        // FIXME
        Ok(Package {
            name: repo.to_string(),
            source: PackageSource::Github {
                owner: owner.to_string(),
                repo: repo.to_string(),
            },
            detail: Some(PackageDetailType::Github {
                release: release.into(),
            }),
            targets: vec![],
        })
    }

    async fn get_release(&self, owner: &str, repo: &str, tag: &str) -> Result<Package> {
        let release = self.github.repo(owner, repo).releases().by_tag(tag).await?;

        //FIXME
        Ok(Package {
            name: repo.to_string(),
            source: PackageSource::Github {
                owner: owner.to_string(),
                repo: repo.to_string(),
            },
            detail: Some(PackageDetailType::Github {
                release: release.into(),
            }),
            targets: vec![],
        })
    }

    async fn download_artifacts<P: AsRef<Path> + Send>(
        &self,
        _release: &Package,
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

        if let Err(_err) = self.fetch_merge_repo(&dir) {
            let _ = remove_dir_all(&dir);
            Repository::clone(&url, dir)?;
            return Ok(());
        }

        Ok(())
    }

    async fn list_managed_releases(&self) -> Result<Vec<Package>> {
        Ok(vec![
            Package {
                name: "hello".to_string(),
                source: PackageSource::Github {
                    owner: "".to_string(),
                    repo: "".to_string(),
                },
                detail: None,
                targets: vec![],
            },
            Package {
                name: "hello2".to_string(),
                source: PackageSource::Github {
                    owner: "".to_string(),
                    repo: "".to_string(),
                },
                detail: None,
                targets: vec![],
            },
        ])
    }
}

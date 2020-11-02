use std::env;
use std::path::Path;

use async_trait::async_trait;
use git2::{RemoteCallbacks, Repository};
use git2::build::RepoBuilder;
use hubcaps::{Credentials, Github};

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

pub(crate) struct GithubClient {
    github: Github,
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
        release: &Package,
        dir: P,
    ) -> Result<()> {
        unimplemented!()
    }

    async fn clone<P: AsRef<Path> + Send>(&self, owner: &str, repo: &str, dir: P) -> Result<()> {
        let url = format!("https://github.com/{}/{}", owner, repo);
        Repository::clone(&url, dir)?;

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

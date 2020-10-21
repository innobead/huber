use std::path::Path;
use async_trait::async_trait;

use hubcaps::{Credentials, Github};

use huber_common::result::Result;
use crate::model::release::{Release, ReleaseDetailType, ReleaseType};

#[async_trait]
trait GithubClientTrait {
    async fn get_latest_release(&self, owner: &str, repo: &str) -> Result<Release>;
    async fn get_release(&self, owner: &str, repo: &str, tag: &str) -> Result<Release>;
    async fn download_artifacts(&self, release: &Release, dir: &Path) -> Result<()>;
}

struct GithubClient {
    github: Github,
}

impl GithubClient {
    fn new(credentials: Option<Credentials>) -> Self {
        Self {
            github: Github::new("huber", credentials).unwrap(),
        }
    }
}

#[async_trait]
impl GithubClientTrait for GithubClient {
    async fn get_latest_release(&self, owner: &str, repo: &str) -> Result<Release> {
        let release = self.github.repo(owner, repo).releases().latest().await?;

        Ok(Release {
            type_: ReleaseType::Github {
                owner: owner.to_string(),
                repo: repo.to_string(),
                url: format!("https://github.com/{}/{}", owner, repo),
            },
            name: repo.to_string(),
            version: release.tag_name.clone(),
            detail: Some(ReleaseDetailType::Github { release: release.into() }),
            // detail: Some(ReleaseDetailType::Github),
            action: None,
        })
    }

    async fn get_release(&self, owner: &str, repo: &str, tag: &str) -> Result<Release> {
        let release = self.github.repo(owner, repo).releases().by_tag(tag).await?;

        Ok(Release {
            type_: ReleaseType::Github {
                owner: owner.to_string(),
                repo: repo.to_string(),
                url: format!("https://github.com/{}/{}", owner, repo),
            },
            name: repo.to_string(),
            version: release.tag_name.clone(),
            detail: Some(ReleaseDetailType::Github { release: release.into() }),
            action: None,
        })
    }

    async fn download_artifacts(&self, release: &Release, dir: &Path) -> Result<()> {
        unimplemented!()
    }
}


use async_trait::async_trait;
use std::path::Path;

use hubcaps::{Credentials, Github};

use huber_common::model::release::{Release, ReleaseDetailType, ReleaseSource};
use huber_common::result::Result;

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
            name: repo.to_string(),
            version: release.tag_name.clone(),
            source: ReleaseSource::Github {
                owner: owner.to_string(),
                repo: repo.to_string(),
            },
            detail: Some(ReleaseDetailType::Github {
                release: release.into(),
            }),
            targets: None,
        })
    }

    async fn get_release(&self, owner: &str, repo: &str, tag: &str) -> Result<Release> {
        let release = self.github.repo(owner, repo).releases().by_tag(tag).await?;

        Ok(Release {
            name: repo.to_string(),
            version: release.tag_name.clone(),
            source: ReleaseSource::Github {
                owner: owner.to_string(),
                repo: repo.to_string(),
            },
            detail: Some(ReleaseDetailType::Github {
                release: release.into(),
            }),
            targets: None,
        })
    }

    async fn download_artifacts(&self, release: &Release, dir: &Path) -> Result<()> {
        unimplemented!()
    }
}

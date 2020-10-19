use std::path::Path;

use hubcaps::{Credentials, Github};

use crate::base::result::Result;
use crate::model::release::{Release, ReleaseDetailType, ReleaseType};

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

impl GithubClient {
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
            // detail: Some(ReleaseDetailType::Github { release }),
            detail: Some(ReleaseDetailType::Github),
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
            // detail: Some(ReleaseDetailType::Github { release }),
            detail: Some(ReleaseDetailType::Github),
            action: None,
        })
    }

    fn download_release_artifacts(&self, repo: &str, version: &str, dest_dir: &Path) -> Result<()> {
        unimplemented!()
    }
}

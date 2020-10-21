use hubcaps::releases::Release as HubcapsRelease;
use hubcaps::releases::Asset as HubcapsAsset;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub(crate) enum ReleaseType {
    Github {
        owner: String,
        repo: String,
        url: String,
    },
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) enum ActionType {
    Shell { exec: String, args: Vec<String> },
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) enum ReleaseDetailType {
    Github { release: GithubRelease } // FIXME
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Action {
    pub(crate) install: ActionType,
    pub(crate) uninstall: ActionType,
    pub(crate) pre_install: Option<ActionType>,
    pub(crate) post_install: Option<ActionType>,
    pub(crate) pre_uninstall: Option<ActionType>,
    pub(crate) post_uninstall: Option<ActionType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Release {
    pub(crate) type_: ReleaseType,
    pub(crate) name: String,
    pub(crate) version: String,
    pub(crate) detail: Option<ReleaseDetailType>,
    pub(crate) action: Option<Action>,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct ReleaseInstance {
    release: Release,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct GithubRelease {
    pub(crate) url: String,
    pub(crate) html_url: String,
    pub(crate) assets_url: String,
    pub(crate) upload_url: String,
    pub(crate) tarball_url: String,
    pub(crate) zipball_url: String,
    pub(crate) id: u64,
    pub(crate) tag_name: String,
    pub(crate) target_commitish: String,
    pub(crate) name: String,
    pub(crate) body: String,
    pub(crate) draft: bool,
    pub(crate) prerelease: bool,
    pub(crate) created_at: String,
    pub(crate) published_at: String,
    pub(crate) assets: Vec<GithubAsset>,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct GithubAsset {
    pub(crate) url: String,
    pub(crate) browser_download_url: String,
    pub(crate) id: u64,
    pub(crate) name: String,
    pub(crate) label: Option<String>,
    pub(crate) state: String,
    pub(crate) content_type: String,
    pub(crate) size: u64,
    pub(crate) download_count: u64,
    pub(crate) created_at: String,
    pub(crate) updated_at: String,
}


impl From<HubcapsRelease> for GithubRelease {
    fn from(r: HubcapsRelease) -> Self {
        Self {
            url: r.url,
            html_url: r.html_url,
            assets_url: r.assets_url,
            upload_url: r.upload_url,
            tarball_url: r.tarball_url,
            zipball_url: r.zipball_url,
            id: r.id,
            tag_name: r.tag_name,
            target_commitish: r.target_commitish,
            name: r.name,
            body: r.body,
            draft: r.draft,
            prerelease: r.prerelease,
            created_at: r.created_at,
            published_at: r.published_at,
            assets: r.assets.into_iter().map(|it| GithubAsset::from(it)).collect(),
        }
    }
}

impl From<HubcapsAsset> for GithubAsset {
    fn from(a: HubcapsAsset) -> Self {
        unimplemented!()
    }
}
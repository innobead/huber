use hubcaps::releases::Asset as HubcapsAsset;
use hubcaps::releases::Release as HubcapsRelease;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ReleaseType {
    Github { owner: String, repo: String },
}

impl ReleaseType {
    pub fn url(&self) -> String {
        match self {
            ReleaseType::Github {
                owner: owner,
                repo: repo,
            } => format!("https://github.com/{}/{}", owner, repo),

            _ => "".to_string(),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ReleaseDetailType {
    Github { release: GithubRelease },
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ReleaseTargetType {
    LinuxAmd64Ubuntu(ReleaseManagement),
    LinuxAmd64CentOs(ReleaseManagement),
    LinuxAmd64OpenSuse(ReleaseManagement),
    MacOS(ReleaseManagement),
    Windows(ReleaseManagement),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ReleaseManagement {
    pub artifact_template: String,
    pub install_commands: Vec<String>,
    pub uninstall_commands: Vec<String>,
    pub upgrade_commands: Vec<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Release {
    pub name: String,
    pub version: String,
    pub type_: ReleaseType,
    pub detail: Option<ReleaseDetailType>,
    pub targets: Option<Vec<ReleaseTargetType>>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ReleaseInstance {
    release: Release,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GithubRelease {
    pub url: String,
    pub html_url: String,
    pub assets_url: String,
    pub upload_url: String,
    pub tarball_url: String,
    pub zipball_url: String,
    pub id: u64,
    pub tag_name: String,
    pub target_commitish: String,
    pub name: String,
    pub body: String,
    pub draft: bool,
    pub prerelease: bool,
    pub created_at: String,
    pub published_at: String,
    pub assets: Vec<GithubAsset>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GithubAsset {
    pub url: String,
    pub browser_download_url: String,
    pub id: u64,
    pub name: String,
    pub label: Option<String>,
    pub state: String,
    pub content_type: String,
    pub size: u64,
    pub download_count: u64,
    pub created_at: String,
    pub updated_at: String,
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
            assets: r
                .assets
                .into_iter()
                .map(|it| GithubAsset::from(it))
                .collect(),
        }
    }
}

impl From<HubcapsAsset> for GithubAsset {
    fn from(a: HubcapsAsset) -> Self {
        unimplemented!()
    }
}

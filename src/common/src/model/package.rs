use std::fmt;
use std::fmt::Display;

use hubcaps::releases::Asset as HubcapsAsset;
use hubcaps::releases::Release as HubcapsRelease;
use serde::export::Formatter;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Package {
    pub name: String,
    pub source: PackageSource,
    pub targets: Vec<PackageTargetType>,
    pub detail: Option<PackageDetailType>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Release {
    pub package: Package,
    pub version: String,
    pub is_current: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum PackageSource {
    Github { owner: String, repo: String },
    Helm { registry: String, repo: String },
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum PackageDetailType {
    Github { release: GithubPackage },
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum PackageTargetType {
    LinuxAmd64(PackageManagement),
    LinuxArm64(PackageManagement),
    MacOS(PackageManagement),
    Windows(PackageManagement),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PackageManagement {
    pub artifact_templates: Vec<String>,
    pub checksum: Option<String>,
    pub install_commands: Option<Vec<String>>,
    pub uninstall_commands: Option<Vec<String>>,
    pub upgrade_commands: Option<Vec<String>>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GithubPackage {
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

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PackageIndex {
    pub name: String,
    pub owner: String,
    pub source: String,
}

impl PackageSource {
    pub fn url(&self) -> String {
        match self {
            PackageSource::Github { owner, repo } => {
                format!("https://github.com/{}/{}", owner, repo)
            }

            _ => "".to_string(),
        }
    }

    pub fn owner(&self) -> String {
        match self {
            PackageSource::Github { owner, repo: _ } => format!("{}", owner),
            PackageSource::Helm { registry, repo: _ } => format!("{}", registry),
        }
    }
}

impl From<HubcapsRelease> for GithubPackage {
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
    fn from(_a: HubcapsAsset) -> Self {
        unimplemented!()
    }
}

impl ToString for PackageSource {
    fn to_string(&self) -> String {
        match self {
            PackageSource::Github { .. } => "github".to_string(),
            PackageSource::Helm { .. } => "helm".to_string(),
        }
    }
}

impl Display for Release {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} (version: {}, source: {})",
            self.package.name,
            self.version,
            self.package.source.to_string()
        )
    }
}

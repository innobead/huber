use std::cmp::Ordering;
use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::path::PathBuf;
use std::str::FromStr;
use std::{env, fmt};

use hubcaps::releases::Release as HubcapsRelease;
use log::{error, warn};
use regex::Regex;
use semver::Version;
use serde::{Deserialize, Serialize};

use crate::model::release::{ReleaseKind, VecExtensionTrait};
use crate::result::Result;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Package {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    pub description: Option<String>,
    pub source: PackageSource,
    pub targets: Vec<PackageTargetType>,
    pub detail: Option<PackageDetailType>,
    // display purpose, injected from release
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(skip_deserializing)]
    pub release_kind: Option<ReleaseKind>,
}

unsafe impl Send for Package {}

unsafe impl Sync for Package {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackageSummary {
    pub name: String,
    pub description: Option<String>,
    pub source: Option<String>,
    pub version: Option<String>,
    pub kind: Option<ReleaseKind>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PackageSource {
    Github { owner: String, repo: String },
    Helm { registry: String, repo: String },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PackageDetailType {
    Github { package: GithubPackage },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PackageTargetType {
    LinuxAmd64(PackageManagement),
    LinuxArm64(PackageManagement),
    MacOS(PackageManagement),
    Windows(PackageManagement),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackageManagement {
    pub artifact_templates: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub executable_templates: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub executable_mappings: Option<HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub install_commands: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uninstall_commands: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upgrade_commands: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_version_regex_template: Option<String>,
    // only keep the {version} part
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scan_dirs: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
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
    #[serde(skip_deserializing)]
    #[serde(skip_serializing)]
    pub body: String,
    pub draft: bool,
    pub prerelease: bool,
    pub created_at: String,
    pub published_at: String,
    pub assets: Vec<GithubAsset>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
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

#[derive(Debug, Clone, Serialize, Deserialize)]
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

impl Package {
    pub fn target(&self) -> Result<PackageManagement> {
        // https://doc.rust-lang.org/std/env/consts/index.html
        let os = env::consts::OS;
        let arch = env::consts::ARCH;
        let e = anyhow!("Unsupported OS {} or ARCH {}", os, arch);

        match os {
            "linux" => match arch {
                "x86_64" => self
                    .targets
                    .iter()
                    .find_map(|it| {
                        if let PackageTargetType::LinuxAmd64(m) = it {
                            Some(m.clone())
                        } else {
                            None
                        }
                    })
                    .ok_or(e),

                "aarch64" => self
                    .targets
                    .iter()
                    .find_map(|it| {
                        if let PackageTargetType::LinuxArm64(m) = it {
                            Some(m.clone())
                        } else {
                            None
                        }
                    })
                    .ok_or(e),

                _ => Err(e),
            },

            "macos" => self
                .targets
                .iter()
                .find_map(|it| {
                    if let PackageTargetType::MacOS(m) = it {
                        Some(m.clone())
                    } else {
                        None
                    }
                })
                .ok_or(e),

            "windows" => self
                .targets
                .iter()
                .find_map(|it| {
                    if let PackageTargetType::Windows(m) = it {
                        Some(m.clone())
                    } else {
                        None
                    }
                })
                .ok_or(e),

            _ => Err(e),
        }
    }

    pub fn parse_version_from_tag_name(&self, tag_name: &String) -> Result<String> {
        let version = if let Some(ref str) = self.target()?.tag_version_regex_template {
            let mut version = String::new();
            let regex = Regex::new(&format!(r"{}", str)).unwrap();

            if let Some(c) = regex.captures(tag_name) {
                if let Some(m) = c.get(1) {
                    version = m.as_str().to_string();
                } else {
                    error!(
                        "Failed to capture the version from {} due to the missing captured group 1",
                        tag_name.clone()
                    );
                }
            }

            if !version.is_empty() {
                version
            } else {
                if Version::parse(tag_name.trim_start_matches("v")).is_ok() {
                    warn!(
                        "Failed to capture the version from {}, because the tag name is a qualified version",
                        tag_name
                    );

                    tag_name.clone()
                } else {
                    return Err(anyhow!(
                        "No qualified version captured from tag name. {}",
                        tag_name
                    ));
                }
            }
        } else {
            tag_name.clone()
        };

        Ok(version)
    }

    pub fn get_scan_dirs(&self, pkg_dir: &PathBuf) -> Result<Vec<PathBuf>> {
        let mut scan_dirs = vec![];

        if let Some(extra_scan_dirs) = self.target()?.scan_dirs {
            let mut extra_scan_dirs: Vec<PathBuf> = extra_scan_dirs
                .into_iter()
                .map(|x| pkg_dir.join(x))
                .collect();
            scan_dirs.append(&mut extra_scan_dirs);
        }

        Ok(scan_dirs)
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
            name: r.name.unwrap_or("".to_string()),
            body: r.body.unwrap_or("".to_string()),
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

impl From<hubcaps::releases::Asset> for GithubAsset {
    fn from(a: hubcaps::releases::Asset) -> Self {
        GithubAsset {
            url: a.url,
            browser_download_url: a.browser_download_url,
            id: a.id,
            name: a.name,
            label: a.label,
            state: a.state,
            content_type: a.content_type,
            size: a.size,
            download_count: a.download_count,
            created_at: a.created_at,
            updated_at: a.updated_at,
        }
    }
}

impl Display for Package {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", &self.name)
    }
}

impl Display for PackageSource {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            PackageSource::Github { .. } => write!(f, "{}", "github"),
            PackageSource::Helm { .. } => write!(f, "{}", "helm"),
        }
    }
}

impl PackageSummary {
    pub fn compare(&self, pkg: &PackageSummary) -> Result<Ordering> {
        let v1 = Version::from_str(&self.version.clone().unwrap().trim_start_matches("v"))?;
        let v2 = Version::from_str(&pkg.version.clone().unwrap().trim_start_matches("v"))?;

        Ok(v1.cmp(&v2))
    }
}

impl From<Package> for PackageSummary {
    fn from(p: Package) -> Self {
        PackageSummary {
            name: p.name.clone(),
            description: p.description.clone(),
            source: Some(p.source.url()),
            version: p.version.clone(),
            kind: p.release_kind.clone(),
        }
    }
}

impl VecExtensionTrait for Vec<PackageSummary> {
    fn sort_by_version(&mut self) {
        self.sort_by(|x, y| y.version.cmp(&x.version));
    }
}

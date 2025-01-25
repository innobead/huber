use std::cmp::Ordering;
use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::path::{Path, PathBuf};
use std::str::FromStr;
use std::{env, fmt};

use anyhow::anyhow;
use regex::Regex;
use semver::Version;
use serde::{Deserialize, Serialize};

use crate::model::release::{ReleaseKind, SortModelTrait};
use crate::str::VersionCompareTrait;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Package {
    pub name: String,

    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,

    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    pub source: PackageSource,

    #[serde(default)]
    pub targets: Vec<PackageTargetType>,

    #[serde(skip)]
    #[serde(with = "serde_yaml::with::singleton_map")]
    pub detail: Option<PackageDetailType>,

    #[serde(skip)]
    #[serde(with = "serde_yaml::with::singleton_map")]
    pub release_kind: Option<ReleaseKind>,
}

impl Default for Package {
    fn default() -> Self {
        Self {
            name: "".to_string(),
            version: None,
            description: None,
            source: PackageSource::Github {
                owner: "".to_string(),
                repo: "".to_string(),
            },
            targets: vec![],
            detail: None,
            release_kind: None,
        }
    }
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
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PackageDetailType {
    Github { package: GithubPackage },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PackageTargetType {
    LinuxAmd64(PackageManagement),
    LinuxArm64(PackageManagement),
    MacOSAmd64(PackageManagement),
    MacOSArm64(PackageManagement),
    WindowsAmd64(PackageManagement),
    WindowsArm64(PackageManagement),
    Default(PackageManagement),
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PackageManagement {
    // {version}, {os} can be used in each. Also, an external URL is acceptable
    pub artifact_templates: Vec<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub executable_mappings: Option<HashMap<String, String>>,

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
        }
    }

    pub fn owner(&self) -> String {
        match self {
            PackageSource::Github { owner, repo: _ } => owner.to_string(),
        }
    }
}

impl Package {
    pub fn target(&self) -> anyhow::Result<PackageManagement> {
        let os = env::consts::OS;
        let arch = env::consts::ARCH;

        let default_pkg_mgmt: Option<_> = self.targets.iter().find_map(|it| match it {
            PackageTargetType::Default(m) => Some(m.clone()),
            _ => None,
        });

        self.get_package_management(os, arch, default_pkg_mgmt)
            .ok_or(anyhow!("Unsupported OS {} or ARCH {}", os, arch))
    }

    fn get_package_management(
        &self,
        os: &str,
        arch: &str,
        default_pkg_mgmt: Option<PackageManagement>,
    ) -> Option<PackageManagement> {
        match os {
            "linux" => match arch {
                "x86_64" => self.targets.iter().find_map(|it| match it {
                    PackageTargetType::LinuxAmd64(m) => Some(m.clone()),
                    _ => default_pkg_mgmt.clone(),
                }),
                "aarch64" => self.targets.iter().find_map(|it| match it {
                    PackageTargetType::LinuxArm64(m) => Some(m.clone()),
                    _ => default_pkg_mgmt.clone(),
                }),
                _ => None,
            },
            "macos" => match arch {
                "x86_64" => self.targets.iter().find_map(|it| match it {
                    PackageTargetType::MacOSAmd64(m) => Some(m.clone()),
                    _ => default_pkg_mgmt.clone(),
                }),
                "aarch64" => self.targets.iter().find_map(|it| match it {
                    PackageTargetType::MacOSArm64(m) => Some(m.clone()),
                    _ => default_pkg_mgmt.clone(),
                }),
                _ => None,
            },
            "windows" => match arch {
                "x86_64" => self.targets.iter().find_map(|it| match it {
                    PackageTargetType::WindowsAmd64(m) => Some(m.clone()),
                    _ => default_pkg_mgmt.clone(),
                }),
                _ => None,
            },
            _ => None,
        }
    }

    pub fn parse_version_from_tag_name(&self, tag_name: &String) -> anyhow::Result<String> {
        let mut version = tag_name.clone();

        if let Some(ref template) = self.target()?.tag_version_regex_template {
            let regex = Regex::new(&template.to_string())?;

            if let Some(capture) = regex.captures(tag_name) {
                if let Some(m) = capture.get(1) {
                    version = m.as_str().to_string();
                } else {
                    return Err(anyhow!(
                        "Failed to capture the version from {} via tag_version_regex_template {}",
                        tag_name,
                        template
                    ));
                }
            }

            if Version::parse(version.trim_start_matches("v")).is_err() {
                return Err(anyhow!(
                    "Failed to parse the version {} from tag_name {}",
                    version,
                    tag_name
                ));
            }
        }

        Ok(version)
    }

    pub fn get_scan_dirs(&self, pkg_dir: &Path) -> anyhow::Result<Vec<PathBuf>> {
        let mut scan_dirs = vec![];

        if let Some(extra_scan_dirs) = self.target()?.scan_dirs {
            let mut extra_scan_dirs: Vec<PathBuf> = extra_scan_dirs
                .into_iter()
                .map(|x| {
                    pkg_dir.join(x.replace(
                        "{version}",
                        self.version.as_ref().unwrap().trim_start_matches("v"),
                    ))
                })
                .collect();
            scan_dirs.append(&mut extra_scan_dirs);
        }

        Ok(scan_dirs)
    }
}

impl From<octocrab::models::repos::Release> for GithubPackage {
    fn from(r: octocrab::models::repos::Release) -> Self {
        Self {
            url: r.url.into(),
            html_url: r.html_url.into(),
            assets_url: r.assets_url.into(),
            upload_url: r.upload_url,
            tarball_url: r.tarball_url.map_or("".into(), |x| x.into()),
            zipball_url: r.zipball_url.map_or("".into(), |x| x.into()),
            id: *r.id,
            tag_name: r.tag_name,
            target_commitish: r.target_commitish,
            name: r.name.unwrap_or("".into()),
            body: r.body.unwrap_or("".into()),
            draft: r.draft,
            prerelease: r.prerelease,
            created_at: r.created_at.map_or("".into(), |x| x.to_string()),
            published_at: r.published_at.map_or("".into(), |x| x.to_string()),
            assets: r.assets.into_iter().map(GithubAsset::from).collect(),
        }
    }
}

impl From<octocrab::models::repos::Asset> for GithubAsset {
    fn from(a: octocrab::models::repos::Asset) -> Self {
        GithubAsset {
            url: a.url.to_string(),
            browser_download_url: a.browser_download_url.to_string(),
            id: *a.id,
            name: a.name,
            label: a.label,
            state: a.state,
            content_type: a.content_type,
            size: a.size as u64,
            download_count: a.download_count as u64,
            created_at: a.created_at.to_string(),
            updated_at: a.updated_at.to_string(),
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
            PackageSource::Github { .. } => write!(f, "github"),
        }
    }
}

impl PackageSummary {
    pub fn compare(&self, pkg: &PackageSummary) -> anyhow::Result<Ordering> {
        let v1 = Version::from_str(self.version.clone().unwrap().trim_start_matches("v"))?;
        let v2 = Version::from_str(pkg.version.clone().unwrap().trim_start_matches("v"))?;

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
            kind: p.release_kind,
        }
    }
}

impl SortModelTrait for Vec<PackageSummary> {
    fn sort_by_version(&mut self) {
        self.sort_by(|x, y| {
            y.version
                .as_ref()
                .unwrap()
                .cmp_version(x.version.as_ref().unwrap())
                .unwrap()
        });
    }

    fn sort_by_name(&mut self) {
        self.sort_by(|x, y| x.name.cmp(&y.name))
    }
}

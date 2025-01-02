use std::cmp::Ordering;
use std::fmt;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

use semver::Version;
use serde::{Deserialize, Serialize};

use crate::model::package::{
    GithubAsset, GithubPackage, Package, PackageDetailType, PackageSource,
};
use crate::str::VersionCompareTrait;

pub trait SortModelTrait {
    fn sort_by_version(&mut self);
    fn sort_by_name(&mut self);
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReleaseIndex {
    pub name: String,
    pub version: String,
    pub owner: String,
    pub source: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Release {
    pub name: String,
    pub version: String,
    pub current: bool,
    pub package: Package,
    pub executables: Option<Vec<String>>,
    pub kind: Option<ReleaseKind>,
}

unsafe impl Send for Release {}

unsafe impl Sync for Release {}

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub enum ReleaseKind {
    Draft,
    PreRelease,
    Release,
}

impl Release {
    pub fn compare(&self, pkg: &Release) -> anyhow::Result<Ordering> {
        if Version::parse(&self.version).is_ok() {
            let v1 = Version::from_str(self.version.trim_start_matches("v"))?;
            let v2 = Version::from_str(pkg.version.trim_start_matches("v"))?;

            Ok(v1.cmp(&v2))
        } else {
            Ok(self.version.cmp(&pkg.version))
        }
    }
}

impl Display for Release {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} (version: {}, source: {})",
            self.name,
            self.version,
            self.package.source.to_string()
        )
    }
}

impl From<octocrab::models::repos::Release> for Release {
    fn from(r: octocrab::models::repos::Release) -> Self {
        let release_kind = if r.draft {
            ReleaseKind::Draft
        } else if r.prerelease {
            ReleaseKind::PreRelease
        } else {
            ReleaseKind::Release
        };

        Release {
            name: "".to_string(),
            version: r.tag_name.clone(),
            current: false,
            package: Package {
                name: "".to_string(),
                source: PackageSource::Github {
                    owner: "".to_string(),
                    repo: "".to_string(),
                },
                targets: vec![],
                detail: Some(PackageDetailType::Github {
                    package: GithubPackage {
                        url: r.url.into(),
                        html_url: r.html_url.into(),
                        assets_url: r.assets_url.into(),
                        upload_url: r.upload_url,
                        tarball_url: r.tarball_url.map_or("".to_string(), |s| s.to_string()),
                        zipball_url: r.zipball_url.map_or("".to_string(), |s| s.to_string()),
                        id: *r.id,
                        tag_name: r.tag_name.clone(),
                        target_commitish: r.target_commitish,
                        name: r.name.unwrap_or(String::new()),
                        body: r.body.unwrap_or(String::new()),
                        draft: r.draft,
                        prerelease: r.prerelease,
                        created_at: r.created_at.map_or("".to_string(), |s| s.to_string()),
                        published_at: r.published_at.map_or("".to_string(), |s| s.to_string()),
                        assets: r
                            .assets
                            .into_iter()
                            .map(|it| GithubAsset::from(it))
                            .collect(),
                    },
                }),
                version: Some(r.tag_name.clone()),
                description: None,
                release_kind: Some(release_kind.clone()),
            },
            executables: None,
            kind: Some(release_kind),
        }
    }
}

impl SortModelTrait for Vec<Release> {
    fn sort_by_version(&mut self) {
        self.sort_by(|x, y| y.version.cmp_version(&x.version).unwrap());
    }

    fn sort_by_name(&mut self) {
        self.sort_by(|x, y| x.name.cmp(&y.name));
    }
}

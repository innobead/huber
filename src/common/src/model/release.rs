use std::cmp::Ordering;
use std::fmt;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

use semver::Version;
use serde::{Deserialize, Serialize};

use crate::model::package::{
    GithubAsset, GithubPackage, Package, PackageDetailType, PackageSource,
};
use crate::result::Result;
use crate::str::VersionCompareTrait;

pub trait VecExtensionTrait {
    fn sort_by_version(&mut self);
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
    pub fn compare(&self, pkg: &Release) -> Result<Ordering> {
        let v1 = Version::from_str(self.version.trim_start_matches("v"))?;
        let v2 = Version::from_str(pkg.version.trim_start_matches("v"))?;

        Ok(v1.cmp(&v2))
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

impl From<hubcaps::releases::Release> for Release {
    fn from(r: hubcaps::releases::Release) -> Self {
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
                        url: r.url,
                        html_url: r.html_url,
                        assets_url: r.assets_url,
                        upload_url: r.upload_url,
                        tarball_url: r.tarball_url,
                        zipball_url: r.zipball_url,
                        id: r.id,
                        tag_name: r.tag_name.clone(),
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

impl VecExtensionTrait for Vec<Release> {
    fn sort_by_version(&mut self) {
        self.sort_by(|x, y| y.version.cmp_version(&x.version).unwrap());
    }
}

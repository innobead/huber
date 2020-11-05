use std::fmt::Formatter;

use serde::export::fmt;
use serde::export::fmt::Display;
use serde::{Deserialize, Serialize};

use crate::model::package::{
    GithubAsset, GithubPackage, Package, PackageDetailType, PackageSource,
};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ReleaseIndex {
    pub name: String,
    pub version: String,
    pub owner: String,
    pub source: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Release {
    pub package: Package,
    pub version: String,
    pub is_current: bool,
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

impl From<hubcaps::releases::Release> for Release {
    fn from(r: hubcaps::releases::Release) -> Self {
        Release {
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
                    },
                }),
                version: Some(r.tag_name.clone()),
            },
            version: r.tag_name.clone(),
            is_current: false,
        }
    }
}

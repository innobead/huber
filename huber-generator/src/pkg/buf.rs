use huber_common::model::package::{Package, PackageSource};
use crate::pkg::default_targets;

#[allow(dead_code)]
pub fn release() -> Package {
    Package {
        name: "buf".to_string(),
        source: PackageSource::Github {
            owner: "bufbuild".to_string(),
            repo: "buf".to_string(),
        },
        targets: default_targets(),
        ..Default::default()
    }
}

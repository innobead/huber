use huber_common::model::package::{Package, PackageSource};

use crate::pkg::default_targets;

#[allow(dead_code)]
pub fn release() -> Package {
    Package {
        name: "bat".to_string(),
        source: PackageSource::Github {
            owner: "sharkdp".to_string(),
            repo: "bat".to_string(),
        },
        targets: default_targets(),
        ..Default::default()
    }
}

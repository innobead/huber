use huber_common::model::package::{Package, PackageSource};

use crate::pkg::default_targets;

#[allow(dead_code)]
pub fn release() -> Package {
    Package {
        name: "ali".to_string(),
        source: PackageSource::Github {
            owner: "nakabonne".to_string(),
            repo: "ali".to_string(),
        },
        targets: default_targets(),
        ..Default::default()
    }
}

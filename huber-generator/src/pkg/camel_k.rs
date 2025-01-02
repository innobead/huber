use huber_common::model::package::{Package, PackageSource};

use crate::pkg::default_targets_no_arm;

#[allow(dead_code)]
pub fn release() -> Package {
    Package {
        name: "camel-k".to_string(),
        source: PackageSource::Github {
            owner: "apache".to_string(),
            repo: "camel-k".to_string(),
        },
        targets: default_targets_no_arm(),
        ..Default::default()
    }
}

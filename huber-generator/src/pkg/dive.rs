use huber_common::model::package::{default_targets_no_arm, Package, PackageSource};

#[allow(dead_code)]
pub fn release() -> Package {
    Package {
        name: "dive".to_string(),
        source: PackageSource::Github {
            owner: "wagoodman".to_string(),
            repo: "dive".to_string(),
        },
        targets: default_targets_no_arm(),
        ..Default::default()
    }
}

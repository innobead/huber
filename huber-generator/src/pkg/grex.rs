use huber_common::model::package::{default_targets_no_arm, Package, PackageSource};

#[allow(dead_code)]
pub fn release() -> Package {
    Package {
        name: "grex".to_string(),
        source: PackageSource::Github {
            owner: "pemistahl".to_string(),
            repo: "grex".to_string(),
        },
        targets: default_targets_no_arm(),
        ..Default::default()
    }
}

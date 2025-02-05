use huber_common::model::package::{default_targets_no_arm, Package, PackageSource};

#[allow(dead_code)]
pub fn release() -> Package {
    Package {
        name: "sad".to_string(),
        source: PackageSource::Github {
            owner: "ms-jpq".to_string(),
            repo: "sad".to_string(),
        },
        targets: default_targets_no_arm(),
        ..Default::default()
    }
}

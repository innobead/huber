use huber_common::model::package::{default_targets_no_arm64_arm, Package, PackageSource};

#[allow(dead_code)]
pub fn release() -> Package {
    Package {
        name: "kotlin".to_string(),
        source: PackageSource::Github {
            owner: "JetBrains".to_string(),
            repo: "kotlin".to_string(),
        },
        targets: default_targets_no_arm64_arm(),
        ..Default::default()
    }
}

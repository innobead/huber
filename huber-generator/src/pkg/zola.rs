use huber_common::model::package::{default_targets_no_arm64_arm, Package, PackageSource};

#[allow(dead_code)]
pub fn release() -> Package {
    Package {
        name: "zola".to_string(),
        source: PackageSource::Github {
            owner: "getzola".to_string(),
            repo: "zola".to_string(),
        },
        targets: default_targets_no_arm64_arm(),
        ..Default::default()
    }
}

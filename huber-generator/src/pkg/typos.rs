use huber::model::package::{default_targets_no_arm64_arm, Package, PackageSource};

#[allow(dead_code)]
pub fn release() -> Package {
    Package {
        name: "typos".to_string(),
        source: PackageSource::Github {
            owner: "crate-ci".to_string(),
            repo: "typos".to_string(),
        },
        targets: default_targets_no_arm64_arm(),
        ..Default::default()
    }
}

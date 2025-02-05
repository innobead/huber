use huber_common::model::package::{default_targets_no_arm, Package, PackageSource};

#[allow(dead_code)]
pub fn release() -> Package {
    Package {
        name: "skaffold".to_string(),
        source: PackageSource::Github {
            owner: "GoogleContainerTools".to_string(),
            repo: "skaffold".to_string(),
        },
        targets: default_targets_no_arm(),
        ..Default::default()
    }
}

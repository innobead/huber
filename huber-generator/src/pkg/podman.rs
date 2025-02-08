use huber::model::package::{default_targets_no_arm, Package, PackageSource};

#[allow(dead_code)]
pub fn release() -> Package {
    Package {
        name: "podman".to_string(),
        source: PackageSource::Github {
            owner: "containers".to_string(),
            repo: "podman".to_string(),
        },
        targets: default_targets_no_arm(),
        ..Default::default()
    }
}

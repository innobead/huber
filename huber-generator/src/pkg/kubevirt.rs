use huber::model::package::{default_targets_no_arm, Package, PackageSource};

#[allow(dead_code)]
pub fn release() -> Package {
    Package {
        name: "kubevirt".to_string(),
        source: PackageSource::Github {
            owner: "kubevirt".to_string(),
            repo: "kubevirt".to_string(),
        },
        targets: default_targets_no_arm(),
        ..Default::default()
    }
}

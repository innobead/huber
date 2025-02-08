use huber::model::package::{default_targets_no_arm, Package, PackageSource};

#[allow(dead_code)]
pub fn release() -> Package {
    Package {
        name: "kind".to_string(),
        source: PackageSource::Github {
            owner: "kubernetes-sigs".to_string(),
            repo: "kind".to_string(),
        },
        targets: default_targets_no_arm(),
        ..Default::default()
    }
}

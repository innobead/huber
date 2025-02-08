use huber::model::package::{default_targets_no_arm, Package, PackageSource};

#[allow(dead_code)]
pub fn release() -> Package {
    Package {
        name: "kubestr".to_string(),
        source: PackageSource::Github {
            owner: "kastenhq".to_string(),
            repo: "kubestr".to_string(),
        },
        targets: default_targets_no_arm(),
        ..Default::default()
    }
}

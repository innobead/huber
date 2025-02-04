use huber_common::model::package::{default_targets_no_arm, Package, PackageSource};

#[allow(dead_code)]
pub fn release() -> Package {
    Package {
        name: "helmfile".to_string(),
        source: PackageSource::Github {
            owner: "helmfile".to_string(),
            repo: "helmfile".to_string(),
        },
        targets: default_targets_no_arm(),
        ..Default::default()
    }
}

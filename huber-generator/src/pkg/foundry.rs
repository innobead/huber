use huber_common::model::package::{default_targets_no_arm_windows, Package, PackageSource};

#[allow(dead_code)]
pub fn release() -> Package {
    Package {
        name: "foundry".to_string(),
        source: PackageSource::Github {
            owner: "foundry-rs".to_string(),
            repo: "foundry".to_string(),
        },
        targets: default_targets_no_arm_windows(),
        ..Default::default()
    }
}

use huber_common::model::package::{default_targets_no_arm_windows, Package, PackageSource};

#[allow(dead_code)]
pub fn release() -> Package {
    Package {
        name: "asdf".to_string(),
        source: PackageSource::Github {
            owner: "asdf-vm".to_string(),
            repo: "asdf".to_string(),
        },
        targets: default_targets_no_arm_windows(),
        ..Default::default()
    }
}

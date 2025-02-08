use huber::model::package::{default_targets_no_arm_windows, Package, PackageSource};

#[allow(dead_code)]
pub fn release() -> Package {
    Package {
        name: "zellij".to_string(),
        source: PackageSource::Github {
            owner: "zellij-org".to_string(),
            repo: "zellij".to_string(),
        },
        targets: default_targets_no_arm_windows(),
        ..Default::default()
    }
}
